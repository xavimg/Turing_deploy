use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::Database;
use mongodb::{options::ClientOptions, Client};

lazy_static! {
    pub static ref DATABASE: AsyncOnce<Database> = AsyncOnce::new(initialize());
}

pub async fn initialize () -> Database {
    let uri = concat!("mongodb://", env!("TURING_USERNAME"), ":", env!("TURING_PASSWORD"), "@127.0.0.1:1234/?authSource=admin&readPreference=primary&directConnection=true&ssl=false");
    let client = ClientOptions::parse(uri).await.expect("Error connectiong to MongoDB");
    let client = Client::with_options(client).expect("Error connectiong to MongoDB");
    client.database(env!("TURING_DATABASE"))
}