use serde::{Serialize, Deserialize};
use crate::{Star, Planet};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanetSystem {
    star: Star,
    planets: Vec<Planet>
}

impl PlanetSystem {
    pub fn new (star: Star, planets: Vec<Planet>) -> Self {
        PlanetSystem { star, planets }
    }

    pub fn get_star (&self) -> &Star {
        &self.star
    }

    pub fn get_planets (&self) -> &Vec<Planet> {
        &self.planets
    }
}