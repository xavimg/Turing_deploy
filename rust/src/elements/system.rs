use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::{Star, Planet};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanetSystem {
    #[serde(rename = "_id")]
    id: ObjectId,
    star: Star,
    planets: Vec<Planet>
}

impl PlanetSystem {
    pub fn new (star: Star, planets: Vec<Planet>) -> Self {
        PlanetSystem { id: ObjectId::new(), star, planets }
    }

    pub fn get_star (&self) -> &Star {
        &self.star
    }

    pub fn get_planets (&self) -> &Vec<Planet> {
        &self.planets
    }
}