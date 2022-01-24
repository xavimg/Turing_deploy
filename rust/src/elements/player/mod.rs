use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "_id")]
    id: ObjectId,
    web_id: u64,
    inventory: Inventory,
    health: u8
}

impl Player {
    pub fn new (web_id: u64) -> Self {
        Player {
            id: ObjectId::new(),
            web_id,
            inventory: Inventory::default(),
            health: 100
        }
    }
}

flat_mod!(inventory, resource);