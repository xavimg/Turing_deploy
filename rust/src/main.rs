#![feature(once_cell, const_fn_floating_point_arithmetic, const_mut_refs, const_for, future_join, future_poll_fn, const_maybe_uninit_zeroed, stream_from_iter, untagged_unions)]
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
    simulate_system();
    Ok(())
    /*
    dotenv::dotenv().unwrap();
    match DATABASE.set(initialize_mongo().await) {
        Err(e) => panic!("{e:?}"),
        Ok(_) => CURRENT_LOGGER.log_info("Connected to MongoDB")
    }

    let server = create_http!(
        status, resources, 
        new_user, user_login, user_logout,
        random_system, get_player, get_all_players
    );  

    server.bind(("0.0.0.0", 8080))?.run().await*/
}