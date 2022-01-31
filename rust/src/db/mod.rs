use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::{Database, Collection};
use mongodb::{options::ClientOptions, Client};
use crate::{PlanetSystem, Player};

pub mod cache;
pub mod filter;

lazy_static! {
    pub static ref DATABASE: AsyncOnce<Database> = AsyncOnce::new(initialize());

    pub static ref PLANET_SYSTEMS: AsyncOnce<Collection<PlanetSystem>> = AsyncOnce::new(async {
        let db = DATABASE.get().await;
        db.collection("system")
    });

    pub static ref PLAYERS: AsyncOnce<Collection<Player>> = AsyncOnce::new(async {
        let db = DATABASE.get().await;
        db.collection("player")
    });
}


pub async fn initialize () -> Database {
    match dotenv::dotenv() {
        Err(e) => panic!("{e}"),
        _ => {}
    };
    
    let uri = format!("mongodb://{}:{}@127.0.0.1:1234/?authSource=admin&readPreference=primary&directConnection=true&ssl=false", get_env!("TURING_USERNAME"), get_env!("TURING_PASSWORD"));
    let client = ClientOptions::parse(uri).await.expect("Error connectiong to MongoDB");
    let client = Client::with_options(client).expect("Error connectiong to MongoDB");
    client.database(get_env!("TURING_DATABASE").as_str())
}