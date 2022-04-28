use std::str::FromStr;
use actix_web::{get, Responder, HttpRequest, web::{Path, self}, HttpResponse};
use bson::{oid::ObjectId, doc};
use futures::StreamExt;
use rand::{distributions::{Alphanumeric, DistString}, thread_rng};
use serde_json::{json, Value};
use crate::{PLAYERS, decode_token, CURRENT_LOGGER, Logger, PLANET_SYSTEMS, Either, Player};

#[get("/test/player/{id}")]
pub async fn test_login (_req: HttpRequest, id: web::Path<u64>) -> HttpResponse {
    let id = id.into_inner();
    match Player::from_foreign_id_or_new(id, Alphanumeric.sample_string(&mut thread_rng(), 10)).await {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        _ => {}
    }

    /*let query = bson::to_document(&PlayerToken::Unloged(id)).unwrap();
    let update = bson::to_document(&PlayerToken::Loged(token.clone())).unwrap();

    match PLAYERS.update_one(doc! { "token": query }, move |x| {
        if let PlayerToken::Unloged(this_id) = x.token { return this_id == id };
        false
    }, doc! { "$set": { "token": update } }).await {
        Ok(Some(_)) => HttpResponse::Ok().body(token),
        Ok(None) => HttpResponse::BadRequest().body("No matching player found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}"))
    }*/
    HttpResponse::Ok().finish()
}

#[get("/player")]
pub async fn get_player_me (req: HttpRequest) -> HttpResponse {
    match decode_token(&req) {
        Err(e) => {
            let e = format!("{e}");
            tokio::spawn(CURRENT_LOGGER.log_error(e.clone())); 
            return HttpResponse::BadRequest().body(e.to_string())
        },

        Ok((string, _)) => {
            let body : &str = &string;
            match PLAYERS.find_one(doc! { "token": body }, move |x| x.token.contains(&string)).await {
                Ok(Some(player)) => {                    
                    let player = json!({
                        "_id": player.id,
                        "name": &player.name,
                        "location": player.location,
                        "hp": player.health,
                        "stats": &player.stats,
                        "inventory": &player.inventory,
                        "color": player.color.as_u32()
                    });

                    return HttpResponse::Ok().json(player)
                }

                Ok(None) => HttpResponse::BadRequest().body("No matching player found"),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
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

#[get("/system/players")]
pub async fn system_players (req: HttpRequest) -> HttpResponse {
    match decode_token(&req) {
        Ok((token, _)) => match Player::from_token(token).await {
            Ok(Some(player)) => match PLANET_SYSTEMS.find_one_by_id(player.location.system).await {
                Ok(Some(system)) => {
                    let col = system.get_players_json();
                    HttpResponse::Ok().json(col.collect::<Vec<Value>>().await)
                },
                Ok(None) => return HttpResponse::InternalServerError().finish(),
                Err(Either::Left(e)) => return HttpResponse::InternalServerError().body(e.to_string()),
                Err(Either::Right(e)) => return HttpResponse::BadRequest().body(e.to_string())
            },

            Ok(None) => return HttpResponse::BadRequest().body("No matching player found"),
            Err(Either::Left(e)) => return HttpResponse::InternalServerError().body(e.to_string()),
            Err(Either::Right(e)) => return HttpResponse::BadRequest().body(e.to_string())
        },

        Err(e) => return HttpResponse::BadRequest().body(format!("{e}"))
    }
}