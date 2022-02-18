use std::{sync::Mutex, ops::{Add, Mul, Neg}, collections::LinkedList};
use llml::{vec::{EucVecd2, EucVecf2}, others::One};
use rand::{prelude::{Distribution, ThreadRng}, distributions::{Uniform, uniform::SampleUniform, Standard}, thread_rng};

/// Thread safe sampler for values in a gaussian ditribution.\
/// This implementation is a translation of Java's ```Random.nextGaussian```.\
/// By default, the values are sampled with a mean of ```0.0``` and a standard deviation of ```1.0```
pub struct Gaussian<T: SampleUniform>(Mutex<LinkedList<T>>, Uniform<T>);

impl<T: SampleUniform + One + Neg<Output = T> + Clone> Gaussian<T> where Gaussian<T>: Distribution<T> {
    pub fn new () -> Self {
        let one = T::one();
        Self(Mutex::new(LinkedList::new()), Uniform::<T>::new_inclusive(-one.clone(), one))
    }

    #[inline]
    pub fn random (&self) -> T {
        self.sample(&mut thread_rng())
    }

    #[inline]
    pub fn with (&self, std: T, mean: T) -> T where T: Add<T, Output = T> + Mul<T, Output = T> {
        std * self.random() + mean
    }

    #[inline]
    pub fn sample_with <R: rand::Rng + ?Sized>(&self, rng: &mut R, std: T, mean: T) -> T where T: Add<T, Output = T> + Mul<T, Output = T> {
        std * self.sample(rng) + mean
    }

    #[inline]
    pub fn with_mean (&self, mean: T) -> <T as Add<T>>::Output where T: Add<T> {
        self.random() + mean
    }

    #[inline]
    pub fn sample_with_mean<R: rand::Rng + ?Sized> (&self, rng: &mut R, mean: T) -> <T as Add<T>>::Output where T: Add<T> {
        self.sample(rng) + mean
    }

    #[inline]
    pub fn with_std (&self, std: T) -> <T as Mul<T>>::Output where T: Mul<T> {
        self.random() * std
    }

    #[inline]
    pub fn sample_with_std<R: rand::Rng + ?Sized> (&self, rng: &mut R, std: T) -> <T as Mul<T>>::Output where T: Mul<T> {
        self.sample(rng) * std
    }
}

macro_rules! impl_gaussian {
    ($($ty:ty, $vec:ident),+) => {
        $(
            impl Distribution<$ty> for Gaussian<$ty> {
                fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                    match self.0.try_lock() {
                        Ok(mut lock) => if let Some(res) = lock.pop_front() { return res },
                        _ => {}
                    }

                    loop {
                        let vec = $vec::new([self.1.sample(rng), self.1.sample(rng)]);
                        let s = vec.dot(vec);
                        if s >= 1. || s == 0. { continue }

                        let mul = <$ty>::sqrt(-2. * s.ln() / s);
                        let vec = vec * mul;

                        match self.0.try_lock() {
                            Ok(mut lock) => lock.push_back(vec.x()),
                            _ => {}
                        }

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

pub struct Random<D, R: ?Sized> {
    pub dist: D,
    pub rng: R
}

impl<D, R: rand::Rng> Random<D, R> {
    pub fn new<T> (dist: D, rng: R) -> Self where D: Distribution<T> {
        Self {
            dist,
            rng
        }
    }

    #[inline]
    pub fn sample<T> (&mut self) -> T where D: Distribution<T> {
        self.dist.sample(&mut self.rng)
    }
}

impl<D> Random<D, ThreadRng> {
    #[inline]
    pub fn with_distribution<T> (dist: D) -> Self where D: Distribution<T> {
        Self::new(dist, thread_rng())
    }
}

impl<R: rand::Rng> Random<Standard, R> {
    #[inline]
    pub fn with_rng<T> (rng: R) -> Self where Standard: Distribution<T> {
        Self::new::<T>(Standard, rng)
    }
}

impl Default for Random<Standard, ThreadRng> {
    #[inline]
    fn default() -> Self {
        Self { dist: Standard, rng: thread_rng() }
    }
}