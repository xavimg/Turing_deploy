use actix_web::{HttpRequest, Responder, web};
use serde_json::json;

pub async fn status (req: HttpRequest) -> impl Responder {
    let json = json!({
        "running": true
    });

    web::Json(json)
}