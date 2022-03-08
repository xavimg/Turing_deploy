use std::ops::Deref;
use std::str::FromStr;
use actix_web::{get, Responder, HttpRequest, web::{Path, self}, HttpResponse};
use bson::{oid::ObjectId, doc};
use serde_json::json;
use crate::{PLAYERS, decode_token, CURRENT_LOGGER, PlayerToken, Logger, is_loopback, test_token, PLANET_SYSTEMS, Either};

#[get("/test/player/token/{id}")]
pub async fn test_login (req: HttpRequest, id: web::Path<u64>) -> HttpResponse {
    if !is_loopback(&req) {
        return HttpResponse::Forbidden().finish()
    }

    let (token, body) = test_token(id.into_inner());
    let query = bson::to_document(&PlayerToken::Unloged(body.id)).unwrap();
    let update = bson::to_document(&PlayerToken::Loged(token.clone())).unwrap();

    match PLAYERS.update_one(doc! { "token": query }, doc! { "$set": { "token": update } }).await {
        Ok(Some(_)) => HttpResponse::Ok().json(token),
        Ok(None) => HttpResponse::BadRequest().body("No matching player found"),
        Err(e) => {
            tokio::spawn(CURRENT_LOGGER.log_error(format!("{e}")));
            HttpResponse::InternalServerError().body(format!("{e}"))
        }
    }
}

#[get("/player")]
pub async fn get_player_me (req: HttpRequest) -> HttpResponse {
    match decode_token(&req) {
        Err(e) => {
            let e = format!("{e}");
            tokio::spawn(CURRENT_LOGGER.log_error(e.clone())); 
            return HttpResponse::BadRequest().body(e.to_string())
        },

        Ok((string, token)) => {
            let id = token.claims.id;
            let bson = bson::to_document(&PlayerToken::Loged(string.clone())).unwrap();
            match PLAYERS.find_one(doc! { "token": bson }, move |player| {
                if let PlayerToken::Loged(ref loged) = player.token {
                    return loged == &string
                }

                false
            }).await {
                Ok(Some(player)) => {
                    let location;

                    if let Some(ref loc) = player.location {
                        match PLANET_SYSTEMS.find_one_by_value(&loc.system).await {
                            Err(Either::Left(e)) => { 
                                tokio::spawn(CURRENT_LOGGER.log_error(format!("{e}")));
                                return HttpResponse::InternalServerError().body(e.to_string())
                            },

                            Err(Either::Right(e)) => { 
                                tokio::spawn(CURRENT_LOGGER.log_error(format!("{e}")));
                                return HttpResponse::BadRequest().body(e.to_string())
                            },

                            Ok (system) => location = Some(if let Some(system) = system {
                                json!({
                                    "system": system.deref(),
                                    "pos": loc.position
                                })
                            } else {
                                json!({ "error": "Planetary system not found" })
                            })
                        }
                    } else {
                        location = None
                    }
                    
                    let player = json!({
                        "_id": player.id,
                        "name": &player.name,
                        "location": location,
                        "hp": player.health,
                        "stats": &player.stats,
                        "inventory": &player.inventory,
                        "color": player.color.as_u32()
                    });

                    return HttpResponse::Ok().json(player)
                }

                Ok(None) => HttpResponse::BadRequest().body("No matching player found"),

                Err(Either::Left(e)) => {
                    tokio::spawn(CURRENT_LOGGER.log_error(e.to_string()));
                    HttpResponse::InternalServerError().body(e.to_string())
                },

                Err(Either::Right(e)) => {
                    tokio::spawn(CURRENT_LOGGER.log_warning(e.to_string()));
                    HttpResponse::BadRequest().body(e.to_string())
                }
            }
        }
    }
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
                "location": &player.location,
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