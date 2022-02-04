use std::{ops::Deref, str::FromStr, sync::Arc};
use actix_web::{Responder, web, HttpRequest, post, get, HttpResponse};
use bson::oid::ObjectId;
use futures::StreamExt;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::{bson::{doc}};
use serde_json::{json, Value};
use strum::IntoEnumIterator;
use crate::{DATABASE, Resource, PLAYERS, Player, PlanetSystem, PlayerToken, PlayerTokenLoged, Logger, CURRENT_LOGGER, Gaussian};
use rand::{thread_rng, prelude::Distribution};

// OUT API
#[get("/status")]
pub async fn status () -> impl Responder {
    let ping = match DATABASE.get().unwrap().run_command(doc! { "ping": 1u32 }, None).await {
        Err(x) => { CURRENT_LOGGER.async_log_error(x); false },
        Ok(x) => match x.get_f64("ok") {
            Ok(x) => {
                let status = x == 1.;
                CURRENT_LOGGER.async_log_info(format!("Status is {x}"));
                status
            },
            Err(x) => { CURRENT_LOGGER.async_log_error(x); false },
        }
    };

    let json = json!({
        "running": true,
        "database": ping
    });

    web::Json(json)
}

#[get("/resources")]
pub async fn resources () -> impl Responder {
    let resources : Vec<Value> = Resource::iter()
        .map(|x: Resource| json!({
            "name": format!("{:?}", x),
            "size": x.get_size(),
            "type": x.get_type()
        }))
        .collect();

    web::Json(json!({
        "resources": resources
    }))
}


// IN API
#[post("/internal/user/signup")]
pub async fn new_user (_: HttpRequest, body: web::Json<u64>) -> impl Responder {
    // TODO INTERNAL IP ONLY
    let valid = match PLAYERS.insert_one(Player::new(PlayerToken::Unloged(body.0))).await {
        Err(x) => { CURRENT_LOGGER.async_log_error(x); false },
        Ok(_) => true
    };

    web::Json(json!({ "valid": valid }))
}

#[post("/internal/user/signin")]
pub async fn user_login (_: HttpRequest, body: web::Json<String>) -> impl Responder {
    let secret = get_env!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_ref());

    let json = match decode::<PlayerTokenLoged>(body.as_str(), &key, &Validation::default()) {
        Err(e) => { CURRENT_LOGGER.async_log_error(e); json!({ "valid": false }) },
        Ok(token) => {
            let body = token.claims;
            let query = bson::to_document(&PlayerToken::Unloged(body.id)).unwrap();
            let update = bson::to_document(&PlayerToken::Loged(body)).unwrap();
        
            let valid = match PLAYERS.update_one(doc! { "token": query }, doc! { "$set": { "token": update } }).await {
                Ok(_) => true,
                Err(e) => { CURRENT_LOGGER.async_log_error(e); false }
            };

            json!({ "valid": valid })
        }
    };

    web::Json(json)
}

#[post("/internal/user/signout")]
pub async fn user_logout (_: HttpRequest, body: web::Json<String>) -> impl Responder {
    let secret = get_env!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_ref());

    let json = match decode::<PlayerTokenLoged>(body.as_str(), &key, &Validation::default()) {
        Err(e) => { CURRENT_LOGGER.async_log_error(e); json!({ "valid": false }) },
        Ok(token) => {
            let body = token.claims;
            let update = bson::to_document(&PlayerToken::Unloged(body.id)).unwrap();
            let query = bson::to_document(&PlayerToken::Loged(body)).unwrap();
        
            let valid = match PLAYERS.update_one(doc! { "token": query }, doc! { "$set": { "token": update } }).await {
                Ok(_) => true,
                Err(e) => { CURRENT_LOGGER.async_log_error(e); false }
            };

            json!({ "valid": valid })
        }
    };

    web::Json(json)
}

#[get("/internal/players/")]
pub async fn get_all_players (req: HttpRequest) -> impl Responder {
    if let Some(addr) = req.peer_addr() {
        if !addr.ip().is_loopback() { return HttpResponse::Forbidden().respond_to(&req) }
    }

    let stream = PLAYERS.find_many(doc! {}, |_| true, None).await;
    let players : Vec<Value> = stream.into_iter().map(|x| serde_json::to_value(x.deref()).unwrap()).collect();
    HttpResponse::Ok().json(json!({ "result": players })).respond_to(&req)
}

#[get("/internal/players/{id}")]
pub async fn get_player (req: HttpRequest) -> impl Responder {
    let id;
    match req.match_info().get("id") {
        None => return HttpResponse::BadRequest().json(json!({ "error": "No id provided" })).respond_to(&req),
        Some(x) => id = x
    }

    let oid;
    match ObjectId::parse_str(id) {
        Err(_) => return HttpResponse::BadRequest().json(json!({ "error": "Invalid id" })).respond_to(&req),
        Ok(x) => oid = x
    }

    if let Some(addr) = req.peer_addr() {
        if !addr.ip().is_loopback() { return HttpResponse::Forbidden().respond_to(&req) }
    }

    let result = match PLAYERS.find_one_by_id(oid).await {
        Err(e) => { CURRENT_LOGGER.async_log_error(e); None },
        Ok(value) => value.map(|x| serde_json::to_value(x.deref()).unwrap())
    };

    HttpResponse::Ok().json(json!({ "result": result })).respond_to(&req)
}

#[get("/internal/test/system")]
pub async fn random_system () -> impl Responder {
    let gaussian : Gaussian<f64> = Gaussian::new();
    let system : PlanetSystem = gaussian.sample(&mut thread_rng());
    web::Json(system)
}