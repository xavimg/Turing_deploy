use std::time::Duration;
use bson::oid::ObjectId;
use llml::vec::{EucVecd2};
use serde::{Serialize, Deserialize};
use crate::{utils::Color, G};

#[derive(Debug, Serialize, Deserialize)]
pub struct Planet {
    pub color: Color,
    pub mass: f64,
    pub position: EucVecd2,
    pub velocity: EucVecd2
}

impl Planet {
    pub fn new (color: Color, mass: f64, position: EucVecd2, velocity: EucVecd2) -> Self {
        Self { color, mass, position, velocity }
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
        let dist = other.position - self.position;
        let r2 = dist.dot(dist);
        (G * EucVecd2::new([other.mass, self.mass]) / r2, dist.unit())
    }
}