use std::hash::Hash;
use bson::oid::ObjectId;
use rand::random;
use serde::{Serialize, Deserialize};
use turing_proc::Maybee;
use crate::{cache::MongoDoc, utils::Color};

#[derive(Debug, Serialize, Deserialize, Maybee)]
pub struct Player {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub location: Option<PlayerLocation>,
    pub name: String,
    pub token: PlayerToken,
    pub stats: PlayerStats,
    pub inventory: Inventory,
    pub health: u8,
    pub color: Color
}

impl Player {
    pub fn new<'a> (token: PlayerToken, name: String) -> Self {
        Player {
            id: ObjectId::new(),
            location: None,
            name,
            token,
            stats: PlayerStats::default(),
            inventory: Inventory::default(),
            health: 100,
            color: random()
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