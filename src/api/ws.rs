use std::{sync::Arc};
use std::collections::{HashMap};
use std::lazy::SyncLazy;
use std::ops::Deref;
use actix::{Actor, StreamHandler, WrapFuture, ContextFutureSpawner, Addr, Handler};
use actix_web::{web, Result, HttpRequest, HttpResponse, get};
use actix_web_actors::ws::{self, WsResponseBuilder};
use bson::{doc, oid::ObjectId};
use chrono::{DateTime, Utc};
use futures::{StreamExt, FutureExt};
use llml::vec::EucVecd2;
use serde::ser::{Error};
use serde::{Serialize, Deserialize};
use serde::de::Visitor;
use tokio::sync::RwLock;
use actix::Message;
use serde_json::{json};
use crate::{CURRENT_LOGGER, decode_token, PLAYERS, Either, Logger, Player, PlayerLocation, Color, color_rgba};
use chrono::serde::ts_milliseconds;

static SOCKETS : SyncLazy<RwLock<HashMap<ObjectId, Arc<Addr<WebSocket>>>>> = SyncLazy::new(|| RwLock::new(HashMap::new()));

/// Define HTTP actor
#[derive(Debug, PartialEq, Hash, Eq)]
struct WebSocket {
    player: ObjectId
}

#[derive(Debug)]
enum WebSocketInput {
    Update(ClientPlayerUpdate)
}

#[derive(Debug, Deserialize)]
pub struct ClientPlayerUpdate {
    system: Option<ObjectId>,
    dir: f32,
    #[serde(with = "ts_milliseconds")]
    at: DateTime<Utc>,
    position: EucVecd2
}

impl<'de> Deserialize<'de> for WebSocketInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct LocalVisitor;
        impl<'de> Visitor<'de> for LocalVisitor {
            type Value = WebSocketInput;

            fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                todo!()
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                if let Some((key, id)) = map.next_entry::<String, u8>()? {
                    if key != "id" { return Err(<A::Error as serde::de::Error>::custom("Expected field 'id'")) }

                    if let Some(key) = map.next_key::<String>()? {
                        if key != "body" { return Err(<A::Error as serde::de::Error>::custom("Expected field 'body'")) }
                        return match id {
                            0x00 => Ok(WebSocketInput::Update(map.next_value()?)),
                            _ => todo!()
                        }
                    }
                }

                Err(<A::Error as serde::de::Error>::missing_field("id"))
            }
        }

        deserializer.deserialize_map(LocalVisitor)
    }
}

impl TryFrom<ws::Message> for WebSocketInput {
    type Error = serde_json::Error;

    fn try_from (value: ws::Message) -> Result<Self, Self::Error> {
        match value {
            ws::Message::Binary(bytes) => serde_json::from_reader(bytes.deref()),
            ws::Message::Text(text) => serde_json::from_str(&text),
            other => Err(serde_json::Error::custom(format!("Expected binary or text message, got {:?}", other)))
        }
    }
}

/// Handler for ws::Message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(msg) = msg {
            if let ws::Message::Ping(b) = msg {
                ctx.pong(&b);
                return;
            }

            if let ws::Message::Close(reason) = msg {
                let id = self.player;
                let fut = async move {
                    let mut lock = SOCKETS.write().await;
                    lock.remove(&id);
                        
                    let lock = lock.downgrade();
                    for value in lock.values() {
                        let msg = PlayerExit { player: id };
                        tokio::spawn(value.send(msg));
                    }
                };

                let actor = fut.into_actor(self);
                actor.spawn(ctx);
                ctx.close(reason);
                return;
            }

            match WebSocketInput::try_from(msg) {
                Ok(WebSocketInput::Update(location)) => {
                    let update = match location.system {
                        Some(ref id) => doc! { "location.system": id, "location.position": bson::to_bson(&location.position).unwrap() },
                        None => doc! { "location.position": bson::to_bson(&location.position).unwrap() }
                    };

                    let id = self.player;
                    let dir = location.dir;
                    let at = location.at;

                    let fut = async move {
                        match PLAYERS.update_one(doc! { "_id": id }, move |x| x.id == id, doc! { "$set": update }).await {
                            Ok(Some(result)) => {
                                tokio::spawn(CURRENT_LOGGER.log_info(format!("Successfull update for player {}", id)));
                                let player_moved = PlayerMoved {
                                    player: result.id,
                                    dir,
                                    at,
                                    position: result.location.position
                                };

                                PLAYERS
                                    .find_many(doc! { "location.system": result.location.system, "_id": { "$ne": result.id } }, move |x| x.id != result.id && x.location.system == result.location.system, None)
                                    .for_each_concurrent(None, |x| async move {
                                        if let Some(addr) = SOCKETS.read().await.get(&x.id) {
                                            tokio::spawn(addr.send(player_moved));
                                            /*match addr.send(player_moved).await {
                                                Ok(_) => CURRENT_LOGGER.log_info(format!("Sent player moved to {}", x.id)),
                                                Err(e) => CURRENT_LOGGER.log_error(format!("Failed to send player moved to {}: {}", x.id, e))
                                            };*/
                                        }
                                    }).await;
                            },

                            Ok(None) => { CURRENT_LOGGER.log_warning(format!("Failed to find player {}", id)).await; },
                            Err(e) => { CURRENT_LOGGER.log_error(format!("Error moving player: {e}")).await; }
                        }
                    };

                    let actor = fut.into_actor(self);
                    actor.spawn(ctx);
                },

                x => panic!("{x:?}")
            }
            return;
        }

        // Nothing
        let actor = CURRENT_LOGGER.log_warning(format!("{msg:?}")).map(|_| ()).into_actor(self);
        actor.spawn(ctx);
    }
}

/// Status information for newly connected player
#[derive(Clone, Serialize, Message)]
#[rtype(result = "()")]
pub struct CurrentStatus {
    #[serde(with = "crate::utils::objectid_hex")]
    pub system: ObjectId,
    pub position: EucVecd2,
    pub players: Vec<NewPlayer>
}

impl Handler<CurrentStatus> for WebSocket {
    type Result = ();

    #[inline]
    fn handle(&mut self, msg: CurrentStatus, ctx: &mut Self::Context) -> Self::Result {
        let body = json!({
            "id": 0x12u8,
             "body": msg
         });

        ctx.binary(serde_json::to_vec(&body).unwrap())
    }
}

/// Player displacement update
#[derive(Clone, Copy, Serialize, Message)]
#[rtype(result = "()")]
pub struct PlayerMoved {
    #[serde(with = "crate::utils::objectid_hex")]
    pub player: ObjectId,
    pub dir: f32,
    #[serde(with = "ts_milliseconds")]
    pub at: DateTime<Utc>,
    pub position: EucVecd2
}

impl Handler<PlayerMoved> for WebSocket {
    type Result = ();

    #[inline]
    fn handle(&mut self, msg: PlayerMoved, ctx: &mut Self::Context) -> Self::Result {
        let body = json!({
           "id": 0x10u8,
            "body": msg
        });

        ctx.binary(serde_json::to_vec(&body).unwrap())
    }
}

/// New player update
#[derive(Debug, Clone, Message, Serialize)]
#[rtype(result = "()")]
pub struct NewPlayer {
    #[serde(with = "crate::utils::objectid_hex")]
    pub id: ObjectId,
    pub name: String,
    pub location: PlayerLocation,
    #[serde(with = "color_rgba")]
    pub color: Color
}

impl Handler<NewPlayer> for WebSocket {
    type Result = ();

    #[inline]
    fn handle(&mut self, msg: NewPlayer, ctx: &mut Self::Context) -> Self::Result {
        let body = json!({
           "id": 0x11u8,
            "body": msg
        });

        ctx.binary(serde_json::to_vec(&body).unwrap())
    }
}

/// Player displacement update
#[derive(Clone, Copy, Serialize, Message)]
#[rtype(result = "()")]
pub struct PlayerExit {
    #[serde(with = "crate::utils::objectid_hex")]
    pub player: ObjectId
}

impl Handler<PlayerExit> for WebSocket {
    type Result = ();

    #[inline]
    fn handle(&mut self, msg: PlayerExit, ctx: &mut Self::Context) -> Self::Result {
        let body = json!({
           "id": 0x13u8,
            "body": msg
        });

        ctx.binary(serde_json::to_vec(&body).unwrap())
    }
}

/// Player new connection
#[get("/player/conn")]
pub async fn start_connection (req: HttpRequest, payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let string = match decode_token(&req) {
        Ok((str, _)) => str,
        Err(e) => return Ok(HttpResponse::BadRequest().body(format!("{e}")))
    };
    
    let body : &str = &string;
    let query = PLAYERS.find_one(doc! { "token": body }, move |x| x.token.contains(&string)).await;
    
    return match query {
        Ok(Some(player)) => {
            let actor = WebSocket { player: player.id };
            let builder = WsResponseBuilder::new(actor, &req, payload);

            let (addr, resp) = builder.start_with_addr()?;
            let addr = Arc::new(addr);
            let addr2 = addr.clone();

            // Add address to socket map
            let id = player.id;
            tokio::spawn(async move {
                let mut lock = SOCKETS.write().await;
                lock.insert(id, addr);
            });

            let id = player.id;
            let system = player.location.system;
            let player2 = player.clone();

            // Get current planetary system info and send it to new player
            // and notify players in same system about new user
            let current = PLAYERS.find_many(doc! { "_id": { "$ne": id }, "location.system": system }, move |x| x.id != id && x.location.system == system, None).filter_map(|other| async {              
                let sockets = SOCKETS.read().await;
                if !sockets.contains_key(&other.id) {
                    return None
                }
                
                let other_player = NewPlayer {
                    id: other.id,
                    name: other.name.to_string(),
                    location: other.location,
                    color: Color::clone(&other.color)
                };
                
                let player_info = NewPlayer {
                    id: player.id,
                    name: player.name.clone(),
                    location: player.location.clone(),
                    color: player.color.clone()
                };

                // Inform about new player
                tokio::spawn(async move {
                    let lock = SOCKETS.read().await;
                    if let Some(addr) = lock.get(&other.id) {
                        let addr = addr.clone();
                        CURRENT_LOGGER.log_info("Informing user").await;
                        addr.send(player_info).await;
                    }
                });

                Some(other_player)
            }).collect::<Vec<_>>().await;

            let current = CurrentStatus {
                position: player2.location.position,
                system: player2.location.system,
                players: current
            };

            tokio::spawn(addr2.send(current));
            Ok(resp)
        },

        Ok(None) => Ok(HttpResponse::BadRequest().body("No matching player found")),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("{e}")))
    }
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl WebSocket {
    #[inline]
    pub async fn get_player (&self) -> mongodb::error::Result<Arc<Player>> {
        match PLAYERS.find_one_by_id(self.player).await {
            Ok(Some(x)) => Ok(x),
            Err(Either::Right(e)) => Err(e),
            Ok(None) => panic!("Player not found"),
            Err(Either::Left(e)) => {
                CURRENT_LOGGER.log_error(e).await;
                panic!("Unexpected error")
            },
        }
    }
}