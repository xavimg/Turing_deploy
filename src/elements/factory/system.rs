use crate::{G, Random};
use llml::{others::Complxd};
use rand::{prelude::{Distribution, ThreadRng}, distributions::{Uniform}};
use crate::{Gaussian, PlanetSystem, Star, Planet, loop_clamp, Color};

const SQRT_2_HALH : f64 = std::f64::consts::SQRT_2 / 2.;
const MIN_MASS : f64 = 1.01e-9;
const MAX_MASS : f64 = 0.01146;

/// Creates a planetary system
pub fn create_system () -> PlanetSystem {
    let mut rng = Random::with_distribution::<f64>(Gaussian::new());
    let star : Star = rng.sample();

    loop {
        let count = rng.dist.sample_with(&mut rng.rng, 0.974172620904933, 3.44274809160305).round();
        if count < 1. { continue }

        let count = count as usize;
        let mut planets : Vec<Planet> = Vec::with_capacity(count);
        let mut accum_dist = 0.;

        for i in 0..count {
            planets.push(create_planet(i, &star, &mut rng, &mut accum_dist));
        }

        return PlanetSystem::new(star, planets)
    }
}

fn create_planet (idx: usize, star: &Star, rng: &mut Random<Gaussian<f64>, ThreadRng>, accum_dist: &mut f64) -> Planet {
    let mass = loop_clamp(MIN_MASS, MAX_MASS, || rng.dist.sample_with(&mut rng.rng, 4.01265200872978, 1.81770640331076));
    let radius = mass * 4.799e-4;
    let color : Color = rand::random();
    
    let dist : f64 = rng.sample();
    let dist = *accum_dist + dist.abs();
    *accum_dist += dist + radius;

    let angle = Uniform::new_inclusive(0., std::f64::consts::TAU).sample(&mut rng.rng);
    let position_norm = Complxd::expi(angle);
    let position = dist * position_norm;

    let min_speed = f64::sqrt((G * star.mass) / dist);
    let max_speed = min_speed * std::f64::consts::SQRT_2;
    let avg_speed = min_speed * SQRT_2_HALH;
    let std = (avg_speed - min_speed) / 4.;

    let mut speed = 0.;
    while speed < min_speed || speed > max_speed {
        speed = rng.dist.sample_with(&mut rng.rng, std, avg_speed)
    }

    let velocity = speed * position_norm.conj();
    Planet::new(idx, color, mass, radius, position.into(), velocity.into())
}