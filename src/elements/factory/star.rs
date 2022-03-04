use rand::{prelude::Distribution};
use crate::{Star, Gaussian};

impl Distribution<Star> for Gaussian<f64> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Star {
        let temp = self.sample_with(rng, 1073.60724180104, 2919.21117764983);
        let mass = 0.513829 * f64::exp(0.000114646 * temp);
        Star::new(temp, mass)
    }
}