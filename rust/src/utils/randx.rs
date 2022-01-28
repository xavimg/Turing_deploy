use std::{sync::Mutex, ops::{Add, Mul, Neg},};
use llml::{vec::{EucVecd2, EucVecf2}, others::One};
use rand::{prelude::Distribution, distributions::{Uniform, uniform::SampleUniform}, thread_rng};

/// Thread safe sampler for values in a gaussian ditribution.\
/// This implementation is a translation of Java's ```Random.nextGaussian```.\
/// By default, the values are sampled with a mean of ```0.0``` and a standard deviation of ```1.0```
pub struct Gaussian<T: SampleUniform>(Mutex<Option<T>>, Uniform<T>);

impl<T: SampleUniform + One + Neg<Output = T> + Clone> Gaussian<T> where Gaussian<T>: Distribution<T> {
    pub fn new () -> Self {
        let one = T::one();
        Self(Mutex::new(None), Uniform::<T>::new_inclusive(-one.clone(), one))
    }

    pub fn random (&self) -> T {
        self.sample(&mut thread_rng())
    }

    pub fn with (&self, std: T, mean: T) -> T where T: Add<T, Output = T> + Mul<T, Output = T> {
        std * self.random() + mean
    }

    pub fn sample_with <R: rand::Rng + ?Sized>(&self, rng: &mut R, std: T, mean: T) -> T where T: Add<T, Output = T> + Mul<T, Output = T> {
        std * self.sample(rng) + mean
    }

    pub fn with_mean (&self, mean: T) -> <T as Add<T>>::Output where T: Add<T> {
        self.random() + mean
    }

    pub fn sample_with_mean<R: rand::Rng + ?Sized> (&self, rng: &mut R, mean: T) -> <T as Add<T>>::Output where T: Add<T> {
        self.sample(rng) + mean
    }

    pub fn with_std (&self, std: T) -> <T as Mul<T>>::Output where T: Mul<T> {
        self.random() * std
    }

    pub fn sample_with_std<R: rand::Rng + ?Sized> (&self, rng: &mut R, std: T) -> <T as Mul<T>>::Output where T: Mul<T> {
        self.sample(rng) * std
    }
}

macro_rules! impl_gaussian {
    ($($ty:ty, $vec:ident),+) => {
        $(
            impl Distribution<$ty> for Gaussian<$ty> {
                fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                    let mut lock = self.0.lock().unwrap();
                    match *lock {
                        Some(x) => { *lock = None; return x },
                        None => {}
                    }

                    loop {
                        let vec = $vec::new([self.1.sample(rng), self.1.sample(rng)]);
                        let s = vec.dot(vec);
                        if s >= 1. || s == 0. { continue }

                        let mul = <$ty>::sqrt(-2. * s.ln() / s);
                        let vec = vec * mul;

                        *lock = Some(vec.x());
                        return vec.y()
                    }
                }
            }
        )*
    };
}

impl_gaussian!(
    f32, EucVecf2,
    f64, EucVecd2
);