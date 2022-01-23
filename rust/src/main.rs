#![feature(once_cell, const_fn_floating_point_arithmetic)]
use actix_web::{HttpServer, App, web};

include!("macros.rs");
flat_mod!(utils, elements, consts, api, db);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/status", web::get().to(status))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}