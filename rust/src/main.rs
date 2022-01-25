#![feature(once_cell, const_fn_floating_point_arithmetic, const_mut_refs, const_for)]
use core::panic;
use actix_web::{HttpServer, App};
use llml::vec::EucVecd2;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

include!("macros.rs");
flat_mod!(utils, elements, consts, api, db);

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    let server = create_http!(status, resources, new_user, random_system);  
    server.bind(("0.0.0.0", 8080))?.run().await
}

async fn insert_system () {
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