use std::{time::Duration, sync::Mutex};
use bson::oid::ObjectId;
use llml::vec::EucVecd2;
use rayon::iter::{ParallelIterator, IntoParallelRefIterator, IndexedParallelIterator, IntoParallelRefMutIterator};
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

    fn indexed_planets (&self) -> impl ParallelIterator<Item = (&Planet, usize)> {
        self.planets.par_iter()
            .zip(0..self.planets.len())
    }

    pub fn simulate (&mut self, dt: Duration) {
        let iter = self.indexed_planets()
            .flat_map(|(x, i)| {
                self.indexed_planets().filter_map(move |(y, j)| {
                    if i == j { return None }
                    Some((x, y))
                })
            });


        let mut interplanet_acc = Vec::<Mutex<EucVecd2>>::with_capacity(self.planets.len());
        for planet in self.get_planets() {
            interplanet_acc.insert(planet.id, Mutex::new(planet.calc_acc_star(&self.star)));
        }

        iter.for_each(|(x, y)| {
            let (acc_x, acc_y) = x.calc_acc(y);
            let mut lock = interplanet_acc[x.id].lock().unwrap();
            *lock += acc_x;
            drop(lock);

            let mut lock = interplanet_acc[y.id].lock().unwrap();
            *lock += acc_y;
        });

        self.planets.par_iter_mut()
            .for_each(|planet| {
                let acc = interplanet_acc[planet.id].lock().unwrap();
                planet.accelerate_and_travel(*acc, dt)
            });
    }
}