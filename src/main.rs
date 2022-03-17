#![feature(once_cell, const_fn_floating_point_arithmetic, const_mut_refs, const_for, future_join, future_poll_fn, const_maybe_uninit_zeroed, untagged_unions, fn_traits)]
use api::{route::*, game::*, ws::start_connection};
use actix_web::dev::Service;

include!("macros.rs");
//include!("tests.rs");
flat_mod!(utils, elements, consts, api, db);

//pub const CURRENT_LOGGER : NoLog = NoLog;
pub const CURRENT_LOGGER : ConsoleLog = ConsoleLog;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    match DATABASE.set(initialize_mongo().await.expect("Error connecting to MongoDB")) {
        Err(e) => panic!("{e:?}"),
        Ok(_) => CURRENT_LOGGER.log_info("Connected to MongoDB").await
    }

    let server = create_http!(
        status, resources, new_user, user_login, user_logout,
        start_connection, get_player_me, get_player,
        test_login
    );  

    server.bind(("0.0.0.0", 8080))?.run().await
}