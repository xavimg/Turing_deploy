use llml::{vec::EucVecd2, others::Complxd};
use rand::{prelude::Distribution, distributions::{Standard, Uniform}};
use crate::{Gaussian, PlanetSystem, Star, Planet, loop_clamp, Color, G_TWO};

const MIN_MASS : f64 = 1.01e-9;
const MAX_MASS : f64 = 0.01146;

impl Distribution<PlanetSystem> for Gaussian<f64> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PlanetSystem {
        fn calc_escape_vel (mass: f64, pos: EucVecd2) -> EucVecd2 {
            let norm = pos.norm();
            let escape_scal = f64::sqrt(G_TWO * mass / norm);

            let current_dir = pos / norm;
            let escape_dir = EucVecd2::new([-current_dir.y(), current_dir.x()]);

            escape_dir * escape_scal
        }

        let star : Star = self.sample(rng);

        loop {
            //let count = self.sample_with(rng, 0.974172620904933, 3.44274809160305).round();
            let count : f64 = 1.;
            if count < 1. { continue }

            let count = count.round() as usize;
            let mut planets : Vec<Planet> = Vec::with_capacity(count);
            let mut accum_dist = 0.;

            for i in 0..count {
                let mass = loop_clamp(MIN_MASS, MAX_MASS, || self.sample_with(rng, 4.01265200872978, 1.81770640331076));
                let color : Color = Standard.sample(rng);
                
                //let dist : f64 = self.sample(rng);
                let dist = 1f64;
                //let dist = accum_dist + (dist.abs() + 4.5);
                accum_dist += dist;

                let angle = Uniform::new_inclusive(0., std::f64::consts::TAU).sample(rng);
                let position : EucVecd2 = (dist * Complxd::expi(angle)).into();

                let star_escape = calc_escape_vel(star.mass, position);
                let offset = planets.iter()
                    .map(|x| calc_escape_vel(x.mass, position - x.position))
                    .reduce(|x, y| x + y);

                let velocity = match offset {
                    None => star_escape,
                    Some(x) => star_escape + x
                };

                planets.insert(i, Planet::new(i, color, mass, position, velocity));
            }

            return PlanetSystem::new(star, planets)
        }
    }
}