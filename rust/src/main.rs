#![feature(once_cell, const_fn_floating_point_arithmetic, const_mut_refs, const_for)]
mod tests;
use actix_web::{HttpServer, App};
use actix_web::dev::Service;
use crate::route::*;

include!("macros.rs");
include!("tests.rs");
flat_mod!(utils, elements, consts, api, db);

pub const CURRENT_LOGGER : ConsoleLog = ConsoleLog;

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    dotenv::dotenv().unwrap();
    let server = create_http!(
        status, resources, 
        new_user, user_login, user_logout,
        random_system
    );  

    server.bind(("0.0.0.0", 8080))?.run().await
}