use std::time::Duration;
use llml::vec::{EucVecd2};
use serde::{Serialize, Deserialize};
use crate::{utils::Color, G};

#[derive(Debug, Serialize, Deserialize)]
pub struct Planet {
    color: Color,
    mass: f64,
    position: EucVecd2,
    velocity: EucVecd2
}

impl Planet {
    pub fn new (color: Color, mass: f64, position: EucVecd2, velocity: EucVecd2) -> Self {
        Self { color, mass, position, velocity }
    }

    pub fn get_color (&self) -> &Color {
        &self.color
    }

    pub fn get_mass (&self) -> f64 {
        self.mass
    }

    pub fn get_pos (&self) -> EucVecd2 {
        self.position
    }

    pub fn get_vel (&self) -> EucVecd2 {
        self.velocity
    }

    pub fn accelerate (&mut self, acc: EucVecd2, dt: Duration) {
        self.velocity += acc * dt.as_secs_f64()
    }

    pub fn travel (&mut self, dt: Duration) {
        self.position += self.velocity * dt.as_secs_f64()
    }

    fn accelerate_and_travel (&mut self, acc: EucVecd2, dt: Duration) {
        self.accelerate(acc, dt);
        self.travel(dt)
    }

    /// Returns the acceleration for each element and the direction from ```self```to ```other```
    /// in ```([acc_self, acc_other], dir)```
    fn calc_acc (&self, other: &Self) -> (EucVecd2, EucVecd2) {
        let dist = other.get_pos() - self.get_pos();
        let r2 = dist.dot(dist);

        (G * EucVecd2::new([other.get_mass(), self.get_mass()]) / r2, dist.unit())
    }
}