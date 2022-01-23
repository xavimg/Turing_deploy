use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::{Database, Collection};
use mongodb::{options::ClientOptions, Client};

use crate::PlanetSystem;

lazy_static! {
    pub static ref DATABASE: AsyncOnce<Database> = AsyncOnce::new(initialize());
    pub static ref PLANET_SYSTEM: AsyncOnce<Collection<PlanetSystem>> = AsyncOnce::new(async {
        let db = DATABASE.get().await;
        db.collection("system")
    });
}

pub async fn initialize () -> Database {
    let uri = concat!("mongodb://", env!("TURING_USERNAME"), ":", env!("TURING_PASSWORD"), "@127.0.0.1:1234/?authSource=admin&readPreference=primary&directConnection=true&ssl=false");
    let client = ClientOptions::parse(uri).await.expect("Error connectiong to MongoDB");
    let client = Client::with_options(client).expect("Error connectiong to MongoDB");
    client.database(env!("TURING_DATABASE"))
}