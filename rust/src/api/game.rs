use std::ops::Deref;
use std::str::FromStr;
use actix_web::{get, Responder, HttpRequest, web::{Path, self}};
use bson::{oid::ObjectId, doc};
use jsonwebtoken::encode;
use serde_json::json;
use crate::{PLAYERS, decode_token, CURRENT_LOGGER, PlayerToken, Logger, is_loopback, PlayerTokenLoged};

#[get("/test/player/token/{id}")]
pub async fn test_login (req: HttpRequest, id: web::Path<u64>) -> impl Responder {
    let id = id.into_inner();
    if !is_loopback(&req) {
        return web::Json(json!({ "error": "Invelid address" }))
    }

    let body = PlayerTokenLoged::default_for(id);
    let token = encode(&Header::default(), &body, EncodingKey::from_secret(JWT_SECRET.as_ref()));

    let query = bson::to_document(&PlayerToken::Unloged(body.id)).unwrap();
    let update = bson::to_document(&PlayerToken::Loged(body)).unwrap();

    let output = match PLAYERS.update_one(doc! { "token": query }, doc! { "$set": { "token": update } }).await {
        Ok(_) => json!({ "token": body }),
        Err(e) => {
            let e = format!("{e}");
            tokio::spawn(CURRENT_LOGGER.log_error(e.clone()));
            json!({ "error": e })
        }
    };

    web::Json(output)
}

#[get("/player")]
pub async fn get_player_me (req: HttpRequest) -> impl Responder {
    let output = match decode_token(&req) {
        Err(e) => {
            let e = format!("{e}");
            tokio::spawn(CURRENT_LOGGER.log_error(e.clone())); 
            json!({ "error": e })
        },

        Ok(token) => {
            let id = token.claims.id;
            let bson = bson::to_document(&PlayerToken::Loged(token.claims)).unwrap();
            match PLAYERS.find_one(doc! { "token": bson }, move |player| player.token.get_id() == id).await {
                Ok(player) => match player {
                    Some(player) => json!({ "player": player.deref() }),
                    None => json!({ "error": "Player not found" })
                },

                Err(e) => json!({ "error": format!("{e}") })
            }
        }
    };

    web::Json(output)
}

#[get("/player/{id}")]
pub async fn get_player (id: Path<String>) -> impl Responder {
    let oid;
    match ObjectId::from_str(id.as_str()) {
        Ok(id) => oid = id,
        Err(e) => return web::Json(json!({ "error": format!("{e}") }))
    }

    let response = match PLAYERS.find_one_by_id(oid).await {
        Ok(player) => match player {
            Some(player) => json!({
                "_id": player.id,
                "name": "todo",
                "system": player.system,
                "hp": player.health,
                "level": player.stats.level,
                "color": player.color.as_u32()
            }),

            None => json!({ "error": "Player not found" })
        },

        Err(e) => json!({ "error": format!("{e}") })
    };

    web::Json(response)
}