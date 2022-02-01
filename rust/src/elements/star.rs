use std::{intrinsics::transmute};
use lazy_static::lazy_static;
use llml::{vec::{EucVecd2, EucVecd3}, mat::Matd3};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Serialize, Deserialize};
use turing_proc::Maybee;
use crate::{utils::Color, H, K};

#[derive(Debug, Serialize, Deserialize, Maybee)]
pub struct Star {
    pub color: Color,
    pub temperature: f64,
    pub mass: f64
}

const nanoC : f64 = 2.998e17;
const nanoC2 : f64 = nanoC * nanoC;
const nanoCH : f64 = nanoC * H;
const nanoC2H2 : f64 = 2. * H * nanoC2;

lazy_static! {
    pub static ref RGB_MATRIX : Matd3 = unsafe { Matd3::new([
        0.67, 0.21, 0.15,
        0.33, 0.71, 0.06,
        0., 0.08, 0.79
    ]).inv_unsafe() };

    pub static ref WHITE_SCALE : EucVecd3 = EucVecd3::new([0.3101, 0.3162, 0.3737]);
}

impl Star {
    pub fn new (temperature: f64, mass: f64) -> Self {
        Self { color: Self::calc_color::<500>(temperature), temperature, mass}
    }

    // PRIVATE
    // It's preatty bad, but I belive that's because the integral doesn't have enough accuracy
    pub fn calc_color<const N: usize> (temp: f64) -> Color {
        let xyz = Self::integrate::<N>(temp);
        let xyz = xyz / xyz.sum();

        let (r, g, b) = ((*RGB_MATRIX * xyz) / *WHITE_SCALE).unzip();
        Color::from_f64(r, g, b)
    }

    fn integrate<const N: usize> (temp: f64) -> EucVecd3 {
        // from 380 to 780
        let delta : f64 = 400. / ((N + 1) as f64);

        let xyz = (0..=N).into_par_iter()
            .map(|i| {
                let alpha = 380. + (i as f64) * delta;
                let radiance = Self::spectral_radiance(alpha, temp);
                let vec = EucVecd3::new([Self::x_func(alpha), Self::y_func(alpha), Self::z_func(alpha)]);
                vec * radiance
            })
            .reduce(|| EucVecd3::default(), |x, y| x + y);
    
        xyz * delta
    }

    fn spectral_radiance (lambda: f64, t: f64) -> f64 {
        return nanoC2H2 / (lambda.powi(5) * (nanoCH / (lambda * K * t)).exp())
    }

    fn gaussian_func (x: f64, mu: f64, gamma1: f64, gamma2: f64) -> f64 {
        f64::exp(-(x - mu).powi(2) / (2. * (if x < mu { gamma1 } else { gamma2 }).powi(2)))
    }

    #[cfg(target_feature = "neon")]
    const X_VEC : EucVecd3 = unsafe { transmute([1.056, 0.362, 0.065, 0.]) };

    #[cfg(target_feature = "sse")]
    const X_VEC : EucVecd3 = unsafe { transmute([0., 0.065, 0.362, 1.056]) };

    fn x_func (lambda: f64) -> f64 {
        Self::X_VEC.dot(EucVecd3::new([
            Self::gaussian_func(lambda, 599.8, 37.9, 31.),
            Self::gaussian_func(lambda, 442., 16., 26.7),
            Self::gaussian_func(lambda, 501.1, 20.4, 26.2)
        ]))
    }

    #[cfg(target_feature = "neon")]
    const Y_VEC : EucVecd2 = unsafe { transmute([0.821, 0.286]) };

    #[cfg(target_feature = "sse")]
    const Y_VEC : EucVecd2 = unsafe { transmute([0.286, 0.821]) };

    fn y_func (lambda: f64) -> f64 {
        Self::Y_VEC.dot(EucVecd2::new([
            Self::gaussian_func(lambda, 568.8, 46.9, 40.5),
            Self::gaussian_func(lambda, 530.9, 16.3, 31.1)
        ]))
    }
    
    #[cfg(target_feature = "neon")]
    const Z_VEC : EucVecd2 = unsafe { transmute([1.217, 0.681]) };

    #[cfg(target_feature = "sse")]
    const Z_VEC : EucVecd2 = unsafe { transmute([0.681, 1.217]) };

    fn z_func (lambda: f64) -> f64 {
        Self::Z_VEC.dot(EucVecd2::new([
            Self::gaussian_func(lambda, 437., 11.8, 36.),
            Self::gaussian_func(lambda, 459., 26., 13.8)
        ]))
    }
}

#[test]
fn color () {
    let temp = 7500.; // Sun temp in kelivn
    println!("{:?}", Star::calc_color::<{(u16::MAX as usize) * 2}>(temp))
}