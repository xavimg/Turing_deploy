use std::hash::Hash;

use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use turing_proc::Maybee;

use crate::cache::MongoDoc;

#[derive(Debug, Serialize, Deserialize, Maybee)]
pub struct Player {
    #[serde(rename = "_id")]
    id: ObjectId,
    token: PlayerToken,
    inventory: Inventory,
    health: u8,
}

impl Player {
    pub fn new (token: PlayerToken) -> Self {
        Player {
            id: ObjectId::new(),
            token,
            inventory: Inventory::default(),
            health: 100,
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
flat_mod!(web, inventory, resource);