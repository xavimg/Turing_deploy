use actix_web::{Responder, web, HttpRequest, post, get};
use bson::{bson, Bson, Document};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use lazy_static::{lazy::Lazy, __Deref};
use llml::mat::Matd3;
use mongodb::{bson::{doc}};
use serde::Deserialize;
use serde_json::{json, Value};
use strum::IntoEnumIterator;
use crate::{DATABASE, Resource, PLAYERS, Player, Gaussian, PlanetSystem, PlayerToken, PlayerTokenLoged};
use rand::{random, thread_rng, prelude::Distribution};

// OUT API
#[get("/status")]
pub async fn status () -> impl Responder {
    let db = DATABASE.get().await;    
    let ping = match db.run_command(doc! { "ping": 1u32 }, None).await {
        Err(x) => { eprintln!("{x:?}"); false },
        Ok(x) => match x.get_f64("ok") {
            Ok(x) => x == 1.,
            Err(x) => { eprintln!("{x:?}"); false },
        }
    };

    let json = json!({
        "running": true,
        "database": ping,
        "random": {
            "bool": random::<bool>(),
            "float": random::<f32>(),
            "mat3": random::<Matd3>()
        }
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
    let players = PLAYERS.get().await;
    let valid = match players.insert_one(Player::new(PlayerToken::Unloged(body.0)), None).await {
        Err(x) => { eprintln!("{x:?}"); false },
        Ok(_) => true
    };

    web::Json(json!({ "valid": valid }))
}

#[post("/internal/user/signin")]
pub async fn user_login (_: HttpRequest, body: web::Json<String>) -> impl Responder {
    let secret = get_env!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_ref());

    let json = match decode::<PlayerTokenLoged>(body.as_str(), &key, &Validation::default()) {
        Err(e) => { eprintln!("{e}"); json!({ "valid": false }) },
        Ok(token) => {
            let body = token.claims;
            let players = PLAYERS.get().await;

            let query = bson::to_document(&PlayerToken::Unloged(body.id)).unwrap();
            let update = bson::to_document(&PlayerToken::Loged(body)).unwrap();
        
            let valid = match players.update_one(doc! { "token": query }, doc! { "$set": { "token": update } }, None).await {
                Ok(_) => true,
                Err(e) => { eprintln!("{e}"); false }
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
        Err(e) => { eprintln!("{e}"); json!({ "valid": false }) },
        Ok(token) => {
            let body = token.claims;
            let players = PLAYERS.get().await;

            let update = bson::to_document(&PlayerToken::Unloged(body.id)).unwrap();
            let query = bson::to_document(&PlayerToken::Loged(body)).unwrap();
        
            let valid = match players.update_one(doc! { "token": query }, doc! { "$set": { "token": update } }, None).await {
                Ok(_) => true,
                Err(e) => { eprintln!("{e}"); false }
            };

            json!({ "valid": valid })
        }
    };

    web::Json(json)
}

#[get("/internal/test/system")]
pub async fn random_system () -> impl Responder {
    let gaussian : Gaussian<f64> = Gaussian::new();
    let system : PlanetSystem = gaussian.sample(&mut thread_rng());
    web::Json(system)
}