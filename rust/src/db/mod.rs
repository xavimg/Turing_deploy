pub mod cache;
pub mod filter;
pub mod find;

use std::lazy::SyncLazy;
use std::mem::{size_of};
use mongodb::{Database};
use mongodb::{options::ClientOptions, Client};
use tokio::sync::OnceCell;
use crate::{PlanetSystem, Player, CURRENT_LOGGER, Logger};
use self::cache::DatabaseCache;

const MAX_CACHE_SIZE : usize = 1073741824; // 1 GiB
const MAX_SINGLE_CACHE_SIZE : usize = MAX_CACHE_SIZE / 2;

pub static DATABASE : OnceCell<Database> = OnceCell::const_new();

pub static PLANET_SYSTEMS: SyncLazy<DatabaseCache<PlanetSystem>> = SyncLazy::new(|| {
    let size = MAX_SINGLE_CACHE_SIZE / size_of::<PlanetSystem>();
    CURRENT_LOGGER.async_log_info(format!("Planet System cache Size: {size} elements"));
    DatabaseCache::new(DATABASE.get().unwrap().collection("system"), size)
});

pub static PLAYERS: SyncLazy<DatabaseCache<Player>> = SyncLazy::new(|| {
    let size = MAX_SINGLE_CACHE_SIZE / size_of::<Player>();
    CURRENT_LOGGER.async_log_info(format!("Player cache Size: {size} elements"));
    DatabaseCache::new(DATABASE.get().unwrap().collection("player"), size)
});

pub async fn initialize_mongo () -> Database {    
    let uri = format!("mongodb://{}:{}@127.0.0.1:1234/?authSource=admin&readPreference=primary&directConnection=true&ssl=false", get_env!("TURING_USERNAME"), get_env!("TURING_PASSWORD"));
    let client = ClientOptions::parse(uri).await.expect("Error connectiong to MongoDB");
    let client = Client::with_options(client).expect("Error connectiong to MongoDB");
    client.database(get_env!("TURING_DATABASE").as_str())
}