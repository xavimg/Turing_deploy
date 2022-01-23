use actix_web::{Responder, web};
use mongodb::{bson::{doc}};
use serde_json::json;
use crate::DATABASE;

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