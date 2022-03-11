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
            x => println!("{x:?}"),
        }
    }
}

#[get("/player/conn")]
pub async fn start_connection (req: HttpRequest, payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let string;

    match decode_token(&req) {
        Ok((s, _)) => string = s,
        Err(e) => { tokio::spawn(CURRENT_LOGGER.log_error(format!("{e}"))); return Ok(HttpResponse::BadRequest().body(format!("{e}"))) }
    }
    
    let bson = bson::to_document(&PlayerToken::Loged(string.clone())).unwrap();
    let query = PLAYERS.find_one(doc! { "token": bson }, move |player| {
        if let PlayerToken::Loged(ref a) = player.token { return a == &string }
        false
    }).await;
    
    return match query {
        Ok(Some(player)) => {
            let actor = WebSocket { player: player.id };
            return ws::start(actor, &req, payload)
        },

        Ok(None) => Ok(HttpResponse::BadRequest().body("No matching player found")),
        Err(Either::Right(e)) => Ok(HttpResponse::BadRequest().body(format!("{e}"))),
        Err(Either::Left(e)) => Ok(HttpResponse::InternalServerError().body(format!("{e}")))
    }
}