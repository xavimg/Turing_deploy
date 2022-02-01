use std::{time::Duration, hash::Hash};
use llml::vec::{EucVecd2};
use serde::{Serialize, Deserialize};
use turing_proc::Maybee;
use crate::{utils::Color, G, Star};

#[derive(Debug, PartialEq, Serialize, Deserialize, Maybee)]
pub struct Planet {
    #[serde(rename = "_id")]
    pub id: usize,
    pub color: Color,
    pub mass: f64,
    pub position: EucVecd2,
    pub velocity: EucVecd2
}

impl Eq for Planet {}

impl Hash for Planet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Planet {
    pub fn new (id: usize, color: Color, mass: f64, position: EucVecd2, velocity: EucVecd2) -> Self {
        Self { id, color, mass, position, velocity }
    }

    pub fn accelerate (&mut self, acc: EucVecd2, dt: Duration) {
        self.velocity += acc * dt.as_secs_f64()
    }

    pub fn travel (&mut self, dt: Duration) {
        self.position += self.velocity * dt.as_secs_f64()
    }

    pub fn accelerate_and_travel (&mut self, acc: EucVecd2, dt: Duration) {
        self.accelerate(acc, dt);
        self.travel(dt)
    }

    /// Returns the acceleration for each element and the direction from ```self```to ```other```
    /// in ```(acc_self, acc_other)```
    pub fn calc_acc (&self, other: &Self) -> (EucVecd2, EucVecd2) {
        let dist = other.position - self.position;
        let r2 = dist.dot(dist);

        let dir = dist.unit();
        let acc = G * EucVecd2::new([other.mass, self.mass]) / r2;

        (acc.x() * dir, acc.y() * dir)
    }

    /// Returns the acceleration for each element and the direction from ```self```to ```other```
    pub fn calc_acc_star (&self, other: &Star) -> EucVecd2 {
        let dist = -self.position;
        let r2 = dist.dot(dist);
        let dir = dist.unit();
        
        dir * G * other.mass / r2
    }
}