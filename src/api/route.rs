use actix_web::{Responder, web, HttpRequest, post, get, HttpResponse};
use futures::{TryStreamExt};
use mongodb::{bson::{doc}, options::FindOptions};
use rand::{distributions::{Alphanumeric, DistString}, thread_rng};
use serde_json::{json, Value};
use strum::IntoEnumIterator;
use crate::{DATABASE, Resource, PLAYERS, Player, CURRENT_LOGGER, Logger, decode_token};

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
    let resources = Resource::iter()
        .map(|x: Resource| json!({
            "name": format!("{:?}", x),
            "size": x.get_size(),
            "type": x.get_type()
        })).collect::<Vec<_>>();

    web::Json(json!({
        "resources": resources
    }))
}

// IN API
#[post("/player/signup")]
pub async fn new_user (_: HttpRequest, body: web::Json<u64>) -> HttpResponse {
    match Player::new(body.0, Alphanumeric.sample_string(&mut thread_rng(), 10)).await {
        Ok(Some(player)) => match PLAYERS.insert_one(player).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(x) => HttpResponse::InternalServerError().body(format!("{x}"))
        },
        Ok(None) => HttpResponse::BadRequest().body("Player with same id or name already exists"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}

#[post("/player/signin")]
pub async fn user_login (req: HttpRequest) -> HttpResponse {
    match decode_token(&req) {
        Ok((body, token)) => {
            let xid = bson::to_bson(&token.claims.id).unwrap();
            match PLAYERS.update_one(doc! { "xid": xid }, move |x| x.xid == token.claims.id, doc! { "$set": { "token": body } }).await {
                Ok(Some(_)) => HttpResponse::Ok().finish(),
                Ok(None) => HttpResponse::BadRequest().body("No matching player found"),
                Err(e) => HttpResponse::InternalServerError().body(format!("{e}"))
            }
        },

        Err(e) => HttpResponse::BadRequest().body(format!("{e}"))
    }
}

#[post("/player/signout")]
pub async fn user_logout (req: HttpRequest) -> HttpResponse {
    if let Ok((string, _)) = decode_token(&req) {    
        let body : &str = &string;
        return match PLAYERS.update_one(doc! { "token": body }, move |x| x.token.contains(&string), doc! { "$set": { "token": null } }).await {
            Ok(Some(_)) => HttpResponse::Ok().finish(),
            Ok(None) => HttpResponse::BadRequest().body("No matching player found"),
            Err(e) => HttpResponse::InternalServerError().body(format!("{e}"))
        }
    }

    HttpResponse::BadRequest().body("No authorization key found")
}

#[get("/ranking")]
pub async fn get_ranking () -> HttpResponse {
    // Reasons for uncached request
    // 1. I don't have time to implement cached version
    // 2. No data is updated, so the cache doesn't need to be notified
    // 3. It's not a vital operation
    let mut opts = FindOptions::default();
    opts.limit = Some(10);
    opts.sort = Some(doc! { "points": -1 });

    match PLAYERS.get_collection().find(doc! { "points": { "$exists": true } }, opts).await {
        Ok(mut players) => {
            let mut results = Vec::with_capacity(10);

            loop {
                match players.try_next().await {
                    Ok(Some(player)) => results.push(json!({
                        "id": player.id,
                        "name": player.name,
                        "points": player.points
                    })),
                    Ok(None) => break,
                    Err(e) => return HttpResponse::InternalServerError().body(format!("{e}"))
                }
            }

            HttpResponse::Ok().body(Value::Array(results).to_string())
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}"))
    }
}