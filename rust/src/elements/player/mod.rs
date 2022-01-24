use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "_id")]
    id: ObjectId,
    inventory: Inventory,
    health: f64
}

flat_mod!(inventory, resource);