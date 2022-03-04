use std::ops::Deref;
use std::str::FromStr;
use actix_web::{get, Responder, HttpRequest, web::{Path, self}, HttpResponse};
use bson::{oid::ObjectId, doc};
use serde_json::json;
use crate::{PLAYERS, decode_token, CURRENT_LOGGER, PlayerToken, Logger, is_loopback, test_token, PLANET_SYSTEMS};

#[get("/test/player/token/{id}")]
pub async fn test_login (req: HttpRequest, id: web::Path<u64>) -> impl Responder {
    HttpResponse::Ok().take()
    /*
    if !is_loopback(&req) {
        return web::Json(json!({ "error": "Invelid address" }))
    }

    let (body, token) = test_token(id.into_inner());
    let query = bson::to_document(&PlayerToken::Unloged(body.id)).unwrap();
    let update = bson::to_document(&PlayerToken::Loged(body)).unwrap();

    let output = match PLAYERS.update_one(doc! { "token": query }, doc! { "$set": { "token": update } }).await {
        Ok(_) => json!({ "token": token }),
        Err(e) => {
            let e = format!("{e}");
            tokio::spawn(CURRENT_LOGGER.log_error(e.clone()));
            json!({ "error": e })
        }
    };

    web::Json(output)
    */
}

// eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjo2MCwiZXhwIjoxNjQ1NTI1ODExLCJpYXQiOjE2NDUyNjY2MTEsImlzcyI6IlRlc3RpbmcifQ.6V4CNkDWV5a3lODu5ZFkkG3GDGQgFifMiRZ2KiIMXgI
#[get("/player")]
pub async fn get_player_me (req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().take()
    /*let output = match decode_token(&req) {
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
                    Some(player) => {
                        let location;

                        if let Some(ref loc) = player.location {
                            match PLANET_SYSTEMS.find_one_by_value(&loc.system).await {
                                Err(e) => { 
                                    let e = format!("{e}");
                                    tokio::spawn(CURRENT_LOGGER.log_error(e.clone()));
                                    return web::Json(json!({ "error": e }))
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

                        json!({ "player": player })
                    },
                    
                    None => json!({ "error": "Player not found" })
                },

                Err(e) => json!({ "error": format!("{e}") })
            }
        }
    };

    web::Json(output)*/
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