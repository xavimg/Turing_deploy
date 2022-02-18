use actix_web::{Responder, web, HttpRequest, post, get};
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::{bson::{doc}};
use serde_json::{json, Value};
use strum::IntoEnumIterator;
use crate::{DATABASE, Resource, PLAYERS, Player, PlanetSystem, PlayerToken, PlayerTokenLoged, CURRENT_LOGGER, create_system, Logger};

// OUT API
#[get("/status")]
pub async fn status () -> impl Responder {
    let ping = match DATABASE.get().unwrap().run_command(doc! { "ping": 1u32 }, None).await {
        Err(x) => { tokio::spawn(CURRENT_LOGGER.log_error(x)); false },
        Ok(x) => match x.get_f64("ok") {
            Ok(x) => {
                let status = x == 1.;
                tokio::spawn(CURRENT_LOGGER.log_info(format!("Status is {x}")));
                status
            },
            Err(x) => { tokio::spawn(CURRENT_LOGGER.log_error(x)); false },
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
    let valid = match PLAYERS.insert_one(Player::new(PlayerToken::Unloged(body.0), format!("todo"), None)).await {
        Err(x) => { tokio::spawn(CURRENT_LOGGER.log_error(x)); false },
        Ok(_) => true
    };

    web::Json(json!({ "valid": valid }))
}

#[post("/internal/user/signin")]
pub async fn user_login (_: HttpRequest, body: web::Json<String>) -> impl Responder {
    let secret = get_env!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_ref());

    let json = match decode::<PlayerTokenLoged>(body.as_str(), &key, &Validation::default()) {
        Err(e) => { tokio::spawn(CURRENT_LOGGER.log_error(e)); json!({ "valid": false }) },
        Ok(token) => {
            let body = token.claims;
            let query = bson::to_document(&PlayerToken::Unloged(body.id)).unwrap();
            let update = bson::to_document(&PlayerToken::Loged(body)).unwrap();
        
            let valid = match PLAYERS.update_one(doc! { "token": query }, doc! { "$set": { "token": update } }).await {
                Ok(_) => true,
                Err(e) => { tokio::spawn(CURRENT_LOGGER.log_error(e)); false }
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
        Err(e) => { tokio::spawn(CURRENT_LOGGER.log_error(e)); json!({ "valid": false }) },
        Ok(token) => {
            let body = token.claims;
            let update = bson::to_document(&PlayerToken::Unloged(body.id)).unwrap();
            let query = bson::to_document(&PlayerToken::Loged(body)).unwrap();
        
            let valid = match PLAYERS.update_one(doc! { "token": query }, doc! { "$set": { "token": update } }).await {
                Ok(_) => true,
                Err(e) => { tokio::spawn(CURRENT_LOGGER.log_error(e)); false }
            };

            json!({ "valid": valid })
        }
    };

    web::Json(json)
}

#[get("/internal/test/system")]
pub async fn random_system () -> impl Responder {
    let system : PlanetSystem = create_system().await;
    web::Json(system)
}