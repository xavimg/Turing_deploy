use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
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

flat_mod!(web, inventory, resource);