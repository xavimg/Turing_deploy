use actix::{Actor, StreamHandler};
use actix_web::{web, Result, HttpRequest, HttpResponse, get};
use actix_web_actors::ws::{self};
use bson::{doc, oid::ObjectId};
use crate::{CURRENT_LOGGER, decode_token, PlayerToken, PLAYERS, Either, Logger};

/// Define HTTP actor
struct WebSocket {
    player: ObjectId
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[get("/player/conn")]
pub async fn start_connection (req: HttpRequest, payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let token;
    match decode_token(&req) {
        Ok(t) => token = t,
        Err(e) => { tokio::spawn(CURRENT_LOGGER.log_error(format!("{e}"))); return Ok(HttpResponse::BadRequest().body(format!("{e}"))) }
    }
    
    let id = token.claims.id;
    let bson = bson::to_document(&PlayerToken::Loged(token.claims)).unwrap();

    return match PLAYERS.find_one(doc! { "token": bson }, move |player| player.token.get_id() == id).await {
        Ok(Some(player)) => {
            let actor = WebSocket { player: player.id };
            return ws::start(actor, &req, payload)
        },

        Err(Either::Right(e)) => Ok(HttpResponse::BadRequest().body(format!("{e}"))),
        Err(Either::Left(e)) => Ok(HttpResponse::InternalServerError().body(format!("{e}"))),
        _ => todo!()
    }
}

