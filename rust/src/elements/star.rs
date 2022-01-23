use std::time::Duration;
use llml::vec::{EucVecd2};
use serde::{Serialize, Deserialize};
use crate::utils::Color;
use super::Body;

#[derive(Debug, Serialize, Deserialize)]
pub struct Star {
    color: Color,
    temperature: f64,
    mass: f64,
    position: EucVecd2,
    velocity: EucVecd2
}

impl Star {
    pub fn new (color: Color, temperature: f64, mass: f64, position: EucVecd2, velocity: EucVecd2) -> Self {
        Self { color, temperature, mass, position, velocity }
    }

    pub fn get_color (&self) -> &Color {
        &self.color
    }

    pub fn get_temp (&self) -> f64 {
        self.temperature
    }
}

impl Body for Star {
    fn get_mass (&self) -> f64 {
        self.mass
    }

    fn get_pos (&self) -> EucVecd2 {
        self.position
    }

    fn get_vel (&self) -> EucVecd2 {
        self.velocity
    }

    fn accelerate (&mut self, acc: EucVecd2, dt: Duration) {
        self.velocity += acc * dt.as_secs_f64()
    }

    fn travel(&mut self, dt: Duration) {
        self.position += self.velocity * dt.as_secs_f64()
    }
}