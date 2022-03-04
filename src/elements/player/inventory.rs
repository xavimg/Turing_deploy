use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::Resource;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Inventory {
    contents: HashMap<Resource, u32>
}