use std::{hash::Hash, sync::Arc};
use bson::{oid::ObjectId, doc};
use rand::random;
use serde::{Serialize, Deserialize};
use tokio::task::JoinError;
use turing_proc::Maybee;
use crate::{cache::MongoDoc, utils::Color, PLANET_SYSTEMS, CURRENT_LOGGER, Logger, create_system, PLAYERS, Either};

#[derive(Debug, Serialize, Deserialize, Maybee)]
pub struct Player {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub location: PlayerLocation,
    pub name: String,
    pub token: PlayerToken,
    pub stats: PlayerStats,
    pub inventory: Inventory,
    pub health: u8,
    pub color: Color
}

impl Player {
    pub async fn new (id: u64, name: String) -> Result<Option<Self>, Either<JoinError, mongodb::error::Error>> {
        let name2 = name.clone();
        let bson = bson::to_bson(&PlayerToken::Unloged(id)).unwrap();
        
        match PLAYERS.find_one(doc! { "$or": { "name": name.clone(), "token": bson } }, move |x| {
            if x.name == name2 { return true }
            if let PlayerToken::Unloged(unlogged) = x.token { return unlogged == id; }
            false
        }).await {
            Ok(None) => Ok(Some(Self::new_unchecked(id, name).await)),
            Ok(Some(_)) => Ok(None),
            Err(e) => Err(e)
        }
    }

    pub async fn new_unchecked (id: u64, name: String) -> Self {
        Player {
            id: ObjectId::new(),
            location: PlayerLocation { system: Self::random_system().await, position: random() },
            name,
            token: PlayerToken::Unloged(id),
            stats: PlayerStats::default(),
            inventory: Inventory::default(),
            health: 100,
            color: random()
        }
    }

    #[inline]
    pub async fn from_foreign_id (id: u64) -> Result<Option<Arc<Self>>, Either<JoinError, mongodb::error::Error>> {
        let bson = bson::to_bson(&PlayerToken::Unloged(id)).unwrap();
        PLAYERS.find_one(doc! { "token": bson }, move |x| {
            if let PlayerToken::Unloged(unlogged) = x.token { return unlogged == id }
            false
        }).await
    }

    pub async fn from_foreign_id_or_new (id: u64, name: String) -> Result<Arc<Self>, Either<JoinError, mongodb::error::Error>> {
        match Self::from_foreign_id(id).await {
            Ok(None) => match PLAYERS.insert_one(Self::new_unchecked(id, name).await).await {
                Ok(player) => Ok(player),
                Err(e) => Err(Either::Right(e))
            },

            Ok(Some(x)) => Ok(x),
            Err(e) => Err(e)
        }
    }

    #[inline]
    pub async fn from_token (token: String) -> Result<Option<Arc<Self>>, Either<JoinError, mongodb::error::Error>> {
        let token_clone = token.clone();
        let bson = bson::to_bson(&PlayerToken::Loged(token)).unwrap();

        PLAYERS.find_one(doc! { "token": bson }, move |x| {
            if let PlayerToken::Loged(ref logged) = x.token { return logged == &token_clone }
            false
        }).await
    }

    async fn random_system () -> ObjectId {
        match PLANET_SYSTEMS.find_any_one().await {
            Ok(Some(system)) => system.id,

            Ok(None) => match PLANET_SYSTEMS.insert_one(create_system()).await {
                Ok(system) => system.id,
                Err(e) => {
                    CURRENT_LOGGER.log_error(format!("{e}")).await;
                    panic!("{e}")
                }
            },

            Err(e) => {
                CURRENT_LOGGER.log_error(format!("{e}")).await;
                panic!("{e}")
            }
        }
    }
}

impl MongoDoc for Player {
    fn get_id (&self) -> ObjectId {
        self.id
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Player {}
flat_mod!(stats, web, inventory, resource);