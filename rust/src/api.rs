use actix_web::{Responder, web, HttpRequest, post, HttpResponse};
use mongodb::{bson::{doc}};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use strum::IntoEnumIterator;
use crate::{DATABASE, Resource, PLAYERS, Player};

// OUT API
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
        "database": ping
    });

    web::Json(json)
}

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
#[post("/internal/user")]
pub async fn new_user (_: HttpRequest, body: web::Json<u64>) -> impl Responder {
    // TODO INTERNAL IP ONLY
    let players = PLAYERS.get().await;
    let valid = match players.insert_one(Player::new(body.0), None).await {
        Err(x) => { eprintln!("{x:?}"); false },
        Ok(_) => true
    };

    web::Json(json!({ "valid": valid }))
}