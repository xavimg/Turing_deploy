use std::{time::Duration, sync::{Arc}};
use bson::oid::ObjectId;
use futures::{pin_mut};
use llml::vec::EucVecd2;
use serde::{Serialize, Deserialize};
use turing_proc::Maybee;
use crate::{Star, Planet, cache::MongoDoc};
use std::hash::Hash;
use tokio::{sync::Mutex, task::JoinError};

#[derive(Clone, Debug, Serialize, Deserialize, Maybee)]
pub struct PlanetSystem {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub star: Star,
    pub planets: Vec<Planet>
}

impl PlanetSystem {
    pub fn new (star: Star, planets: Vec<Planet>) -> Self {
        PlanetSystem { id: ObjectId::new(), star, planets }
    }

    fn indexed_planets (&self) -> impl Iterator<Item = (&Planet, usize)> {
        self.planets.iter()
            .zip(0..self.planets.len())
    }

    pub async fn simulate (&'static mut self, dt: Duration) -> Result<(), JoinError> {
        let iter = self.indexed_planets().flat_map(|(x, i)| {
            self.indexed_planets().filter_map(move |(y, j)| {
                if i == j { return None }
                Some((x, y))
            })
        });


        let interplanet_acc = Arc::new(Mutex::new(Vec::<EucVecd2>::with_capacity(self.planets.len())));
        let mut lock = interplanet_acc.lock().await;
        for planet in self.planets.iter() {
            lock.insert(planet.id, planet.calc_acc_star(&self.star));
        }
        drop(lock);

        let handles = iter.map(|(x, y)| {
            let acc_clone = interplanet_acc.clone();
            tokio::spawn(async move {
                let (acc_x, acc_y) = x.calc_acc(y);
                let mut lock = acc_clone.lock().await;
                lock[x.id] += acc_x;
                lock[y.id] += acc_y;
            })
        });

        let join = futures::future::try_join_all(handles).await?;
        let stream = self.planets.iter_mut();

        let lock = interplanet_acc.lock().await;
        stream.for_each(|planet| { planet.accelerate_and_travel(lock[planet.id], dt); });

        Ok(())
    }
}

impl MongoDoc for PlanetSystem {
    fn get_id (&self) -> ObjectId {
        self.id
    }
}

impl PartialEq for PlanetSystem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for PlanetSystem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for PlanetSystem {}