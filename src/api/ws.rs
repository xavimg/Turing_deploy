use std::str::FromStr;
use std::{sync::Arc};
use std::collections::{HashMap, HashSet};
use std::lazy::SyncLazy;
use std::ops::Deref;
use actix::{Actor, StreamHandler, WrapFuture, ContextFutureSpawner, Addr, Handler};
use actix_web::web::Bytes;
use actix_web::{web, Result, HttpRequest, HttpResponse, get};
use actix_web_actors::ws::{self, WsResponseBuilder};
use bson::{doc, oid::ObjectId};
use futures::StreamExt;
use llml::vec::EucVecd2;
use serde::{Serialize, Deserialize, Deserializer};
use serde::de::Visitor;
use tokio::sync::RwLock;
use actix::Message;
use serde_json::{json, Value};
use crate::{CURRENT_LOGGER, decode_token, PlayerToken, PLAYERS, Either, Logger, Player, PlayerLocation, PlanetSystem, PLANET_SYSTEMS, Color, color_rgba};

static SOCKETS : SyncLazy<RwLock<HashMap<ObjectId, Arc<Addr<WebSocket>>>>> = SyncLazy::new(|| RwLock::new(HashMap::new()));

/// Define HTTP actor
#[derive(Debug, PartialEq, Hash, Eq)]
struct WebSocket {
    player: ObjectId
}

#[derive(Debug)]
enum WebSocketInput {
    Update(Value)
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
                            0x00 => Ok(WebSocketInput::Update(map.next_value::<Value>()?)),
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
            _ => todo!()
        }
    }
}

/// Handler for ws::Message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(msg) = msg {
            match WebSocketInput::try_from(msg) {
                Ok(WebSocketInput::Update(location)) => {
                    let id = self.player.clone();
                    let fut = async move {
                        let bson = bson::to_bson(&location).unwrap();
                        match PLAYERS.update_one(doc! { "_id": id }, move |x| x.id == id, doc! { "$set": { "location": bson } }).await {
                            Ok(Some(_)) => {
                                CURRENT_LOGGER.log_info("Successfull change").await;

                                let system = ObjectId::from_str(location["system"]["$oid"].as_str().unwrap()).unwrap();
                                let position = &location["position"];
                                let position = EucVecd2::new([position["x"].as_f64().unwrap(), position["y"].as_f64().unwrap()]);

                                let payload = PlayerMoved {
                                    player: id,
                                    position
                                };

                                let mut players = PLAYERS.find_many(doc! { "location.system": system }, move |x| x.location.system == system, None);
                                let lock = SOCKETS.read().await;

                                while let Some(player) = players.next().await {
                                    if let Some(addr) = lock.get(&player.id) {
                                        let addr = addr.clone();
                                        tokio::spawn(addr.send(payload));
                                    }
                                }
                            },
                            x => panic!("{x:?}")
                        }
                    };

                    let actor = fut.into_actor(self);
                    actor.spawn(ctx);
                },

                x => panic!("{x:?}")
            }
        }

        // Nothing
    }
}

/// Player displacement update
#[derive(Clone, Copy, Message, Serialize)]
#[rtype(result = "()")]
pub struct PlayerMoved {
    pub player: ObjectId,
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
    #[serde(rename = "_id")]
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

/// Player new connection
#[get("/player/conn")]
pub async fn start_connection (req: HttpRequest, payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let string= match decode_token(&req) {
        Ok((str, _)) => str,
        Err(e) => return Ok(HttpResponse::BadRequest().body(format!("{e}")))
    };
    
    let bson = bson::to_document(&PlayerToken::Loged(string.clone())).unwrap();
    let query = PLAYERS.find_one(doc! { "token": bson }, move |player| {
        if let PlayerToken::Loged(ref a) = player.token { return a == &string }
        false
    }).await;
    
    return match query {
        Ok(Some(player)) => {
            let actor = WebSocket { player: player.id };
            let builder = WsResponseBuilder::new(actor, &req, payload);
            let (addr, resp) = builder.start_with_addr()?;
            let addr = Arc::new(addr);

            // Add address to socket map
            let id = player.id;
            tokio::spawn(async move {
                let mut lock = SOCKETS.write().await;
                lock.insert(id, addr);
            });

            // Notify players in same system about new user
            // TODO spawn on tokio
            PLAYERS.find_many(doc! { "location.system": player.location.system }, move |x| x.location.system == player.location.system, None).for_each_concurrent(None, |player: Arc<Player>| async move {
                if player.id == id { return; }
                let new_player = NewPlayer {
                    id: player.id,
                    name: player.name.clone(),
                    location: player.location.clone(),
                    color: player.color.clone()
                };

                let lock = SOCKETS.read().await;
                if let Some(addr) = lock.get(&player.id) {
                    let addr = addr.clone();
                    CURRENT_LOGGER.log_info("Informing user").await;
                    tokio::spawn(addr.send(new_player.clone()));
                }
            }).await;

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