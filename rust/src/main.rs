#![feature(once_cell, const_fn_floating_point_arithmetic)]
use core::panic;

use actix_web::{HttpServer, App, web};
use llml::vec::EucVecd2;

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

#[test]
fn pasre_resource () {
    let resource = serde_json::to_string(&Iron).unwrap();
    println!("{resource}")
}

async fn insert_sun () {
    let systems = PLANET_SYSTEMS.get().await;
    let sun = Star::new(5772., 1048.);
    let earth = Planet::new(Color::BLUE, 0.003146, EucVecd2::new([1., 0.]), EucVecd2::new([0., 2e-7]));

    let test = PlanetSystem::new(sun, vec![earth]);
    match systems.insert_one(&test, None).await {
        Err(x) => panic!("{x:?}"),
        Ok(x) => println!("{x:?}")
    }
}

async fn get_system () -> Option<PlanetSystem> {
    let systems = PLANET_SYSTEMS.get().await;
    match systems.find_one(None, None).await {
        Err(x) => panic!("{x:?}"),
        Ok(x) => x
    }
}