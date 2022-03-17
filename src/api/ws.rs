use std::{sync::Arc};
use std::ops::Deref;
use actix::{Actor, StreamHandler, WrapFuture, ContextFutureSpawner};
use actix_web::{web, Result, HttpRequest, HttpResponse, get};
use actix_web_actors::ws::{self};
use bson::{doc, oid::ObjectId};
use llml::vec::EucVecd2;
use serde::Deserialize;
use serde::de::Visitor;
use crate::{CURRENT_LOGGER, decode_token, PlayerToken, PLAYERS, Either, Logger, Player};

/// Define HTTP actor
struct WebSocket {
    player: ObjectId
}

#[derive(Debug)]
enum WebSocketInput {
    Update(WebSocketUpdate)
}

#[derive(Debug, Deserialize)]
struct WebSocketUpdate {
    #[serde(skip_deserializing)] 
    system: ObjectId,
    x: f64,
    y: f64
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
                            0x00 => Ok(WebSocketInput::Update(map.next_value::<WebSocketUpdate>()?)),
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
                Ok(WebSocketInput::Update(WebSocketUpdate { x, y, .. })) => {
                    let id = self.player.clone();
                    let fut = async move { 
                        let position = bson::to_bson(&EucVecd2::new([x, y])).unwrap();
                        match PLAYERS.update_one(doc! { "_id": id }, move |x| x.id == id, doc! { "$set": { "location.position": position } }).await {
                            Ok(Some(_)) => CURRENT_LOGGER.log_info("Successfull update").await,
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

#[get("/player/conn")]
pub async fn start_connection (req: HttpRequest, payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let string;

    match decode_token(&req) {
        Ok((s, _)) => string = s,
        Err(e) => return Ok(HttpResponse::BadRequest().body(format!("{e}")))
    }
    
    let bson = bson::to_document(&PlayerToken::Loged(string.clone())).unwrap();
    let query = PLAYERS.find_one(doc! { "token": bson }, move |player| {
        if let PlayerToken::Loged(ref a) = player.token { return a == &string }
        false
    }).await;
    
    return match query {
        Ok(Some(player)) => {
            let actor = WebSocket { 
                player: player.id 
            };

            return ws::start(actor, &req, payload)
        },

        Ok(None) => Ok(HttpResponse::BadRequest().body("No matching player found")),
        Err(Either::Right(e)) => Ok(HttpResponse::BadRequest().body(format!("{e}"))),
        Err(Either::Left(e)) => Ok(HttpResponse::InternalServerError().body(format!("{e}")))
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