pub mod cache;
pub mod filter;
pub mod find_many;

use std::lazy::SyncLazy;
use std::mem::{size_of};
use mongodb::{Database};
use mongodb::{options::ClientOptions, Client};
use tokio::sync::OnceCell;
use crate::{PlanetSystem, Player};
use self::cache::CollectionCache;

const MAX_CACHE_SIZE : usize = 1073741824; // 1 GiB
const MAX_SINGLE_CACHE_SIZE : usize = MAX_CACHE_SIZE / 2;
pub static DATABASE : OnceCell<Database> = OnceCell::const_new();

pub static PLANET_SYSTEMS: SyncLazy<CollectionCache<PlanetSystem>> = SyncLazy::new(|| {
    let size = MAX_SINGLE_CACHE_SIZE / size_of::<PlanetSystem>();
    CollectionCache::new(DATABASE.get().unwrap().collection("system"), size)
});

pub static PLAYERS: SyncLazy<CollectionCache<Player>> = SyncLazy::new(|| {
    let size = MAX_SINGLE_CACHE_SIZE / size_of::<Player>();
    CollectionCache::new(DATABASE.get().unwrap().collection("player"), size)
});

#[cfg(debug_assertions)]
macro_rules! mongo_port { () => { "127.0.0.1:1234" } }

#[cfg(not(debug_assertions))]
macro_rules! mongo_port { () => { "123.20.0.2:27017" } }

pub async fn initialize_mongo () -> mongodb::error::Result<Database> {    
    let uri = format!("mongodb://{}:{}@{}/?authSource=admin&readPreference=primary&directConnection=true&ssl=false", get_env!("TURING_USERNAME"), get_env!("TURING_PASSWORD"), mongo_port!());
    println!("{uri}");

    let client = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client)?;
    Ok(client.database(get_env!("TURING_DATABASE").as_str()))
}