use std::{time::Duration, hash::Hash, collections::{HashMap}};
use llml::vec::{EucVecd2};
use rand::{distributions::{WeightedIndex, WeightedError}, prelude::Distribution, thread_rng};
use serde::{Serialize, Deserialize};
use turing_proc::Maybee;
use crate::{utils::Color, G, Star, Resource};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Maybee)]
pub struct Planet {
    #[serde(rename = "_id")]
    pub id: usize,
    pub color: Color,
    pub mass: f64,
    pub radius: f64,
    pub position: EucVecd2,
    pub velocity: EucVecd2,
    pub resources: Vec<Resource>,
    pub resource_weights: WeightedIndex<f64>
}

impl Hash for Planet {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Planet {}

impl Planet {
    pub fn new (id: usize, color: Color, mass: f64, radius: f64, position: EucVecd2, velocity: EucVecd2, resources: HashMap<Resource, f64>) -> Result<Self, WeightedError> {
        let resource_weights = WeightedIndex::new(resources.values())?;
        let resources = resources.into_keys().collect::<Vec<_>>();

        Ok(Self { id, color, mass, position, velocity, radius, resources, resource_weights })
    }

    pub fn accelerate (&mut self, acc: EucVecd2, dt: Duration) {
        self.velocity += acc * dt.as_secs_f64()
    }

    pub fn travel (&mut self, dt: Duration) {
        self.position += self.velocity * dt.as_secs_f64()
    }

    #[inline]
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

    #[inline]
    pub fn peek_resource (&self) -> &Resource {
        let idx = self.resource_weights.sample(&mut thread_rng());
        unsafe { self.resources.get_unchecked(idx) }
    }
}