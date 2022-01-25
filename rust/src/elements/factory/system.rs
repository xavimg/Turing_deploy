use llml::{vec::EucVecd2, others::Complxf};
use rand::{prelude::Distribution, distributions::Standard};
use crate::{Gaussian, PlanetSystem, Star, Planet, loop_clamp, Color, G, G_TWO};

const MIN_MASS : f64 = 1.01e-9;
const MAX_MASS : f64 = 0.01146;

impl Distribution<PlanetSystem> for Gaussian<f64> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PlanetSystem {
        fn calc_escape_vel (mass: f64, pos: EucVecd2) -> EucVecd2 {
            let escape_scal = f64::sqrt(G_TWO * mass / pos.norm());
            let escape_rel = pos.y() / pos.x();
            let escape_alpha = f64::sqrt(escape_rel * escape_rel + 1.);
            let escape_dir = EucVecd2::new([1., escape_rel]) / escape_alpha;
            escape_dir * escape_scal
        }

        let star : Star = self.sample(rng);

        loop {
            let count = self.sample_with(rng, 0.974172620904933, 3.44274809160305).round();
            if count < 1. {
                continue
            }

            let count = count as usize;
            let mut planets : Vec<Planet> = Vec::with_capacity(count);
            
            for i in 0..count {
                let mass = loop_clamp(MIN_MASS, MAX_MASS, || self.sample_with(rng, 4.01265200872978, 1.81770640331076));
                let color : Color = Standard.sample(rng);
                let position : EucVecd2 = Standard.sample(rng);

                let star_escape = calc_escape_vel(star.mass, position);
                let offset : Option<EucVecd2> = planets.iter()
                    .map(|x| calc_escape_vel(x.mass, position - x.position))
                    .reduce(|x, y| x + y);

                let velocity = match offset {
                    None => star_escape,
                    Some(x) => star_escape + x
                };

                planets.insert(i, Planet::new(color, mass, position, velocity));
            }

            return PlanetSystem::new(star, planets)
        }
    }
}