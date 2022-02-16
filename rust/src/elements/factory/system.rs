use std::{time::Duration, intrinsics::transmute, cmp::Ordering, ops::DerefMut};
use crate::{Durationx, G};
use llml::{vec::EucVecd2, others::Complxd};
use rand::{prelude::{Distribution, ThreadRng}, distributions::{Standard, Uniform}, thread_rng};
use tokio::{task::JoinError};
use crate::{Gaussian, PlanetSystem, Star, Planet, loop_clamp, Color};

const SQRT_2_HALH : f64 = std::f64::consts::SQRT_2 / 2.;
const MIN_MASS : f64 = 1.01e-9;
const MAX_MASS : f64 = 0.01146;


/// Creates a planetary system
pub async fn create_system () -> PlanetSystem {
    let mut thread_rng = thread_rng();
    let gaussian = Gaussian::new();
    let star : Star = gaussian.sample(&mut thread_rng);

    loop {
        let count = gaussian.sample_with(&mut thread_rng, 0.974172620904933, 3.44274809160305).round();
        //let count : f64 = 2.;
        if count < 1. { continue }

        let count = count.round() as usize;
        let mut planets : Vec<Planet> = Vec::with_capacity(count);
        let mut accum_dist = 0.;

        for i in 0..count {
            let mass = loop_clamp(MIN_MASS, MAX_MASS, || gaussian.sample_with(&mut thread_rng, 4.01265200872978, 1.81770640331076));
            let color : Color = Standard.sample(&mut thread_rng);
            
            let dist : f64 = gaussian.sample(&mut thread_rng);
            let dist = accum_dist + dist.abs();
            accum_dist += dist;
            println!("{dist:?}");

            let angle = Uniform::new_inclusive(0., std::f64::consts::TAU).sample(&mut thread_rng);
            let position_norm = Complxd::expi(angle);
            let position = dist * position_norm;

            let min_speed = f64::sqrt((G * star.mass) / dist);
            let max_speed = min_speed * std::f64::consts::SQRT_2;
            let avg_speed = min_speed * SQRT_2_HALH;
            let std = (avg_speed - min_speed) / 4.;

            let mut speed = 0.;
            while speed < min_speed || speed > max_speed {
                speed = gaussian.sample_with(&mut thread_rng, std, avg_speed)
            }

            let velocity = speed * position_norm.conj();
            planets.push(Planet::new(i, color, mass, position.into(), velocity.into()));
        }

        return PlanetSystem::new(star, planets)
    }
}