use std::mem::size_of;

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::{Database};
use mongodb::{options::ClientOptions, Client};
use crate::{PlanetSystem, Player, CURRENT_LOGGER, Logger};
use self::cache::DatabaseCache;

pub mod cache;
pub mod filter;
pub mod field;

const MAX_CACHE_SIZE : usize = 1073741824; // 1 GiB
const CACHE_COUNT : usize = 2;
const MAX_SINGLE_CACHE_SIZE : usize = MAX_CACHE_SIZE / CACHE_COUNT;

lazy_static! {
    pub static ref DATABASE: AsyncOnce<Database> = AsyncOnce::new(initialize());

    pub static ref PLANET_SYSTEMS: AsyncOnce<DatabaseCache<PlanetSystem>> = AsyncOnce::new(async {
        let db = DATABASE.get().await;
        let size = MAX_SINGLE_CACHE_SIZE / size_of::<PlanetSystem>();

        CURRENT_LOGGER.async_log_info(format!("Planet System cache Size: {size} elements"));
        DatabaseCache::new(db.collection("system"), size)
    });

    pub static ref PLAYERS: AsyncOnce<DatabaseCache<Player>> = AsyncOnce::new(async {
        let db = DATABASE.get().await;
        let size = MAX_SINGLE_CACHE_SIZE / size_of::<Player>();

        CURRENT_LOGGER.async_log_info(format!("Player cache Size: {size} elements"));
        DatabaseCache::new(db.collection("player"), size)
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