use crate::Resource;
use super::*;
use serde::{Serialize, Deserialize};

macro_rules! impl_type { 
    ($size:literal, $($target:ident),+) => {
        $(
            impl_type!($target, $size, None, None);
        )*
    };

    ($target:ident, $size:literal, $prob:expr, $price:expr) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $target;

        impl<'a> Resource<'a> for $target {
            fn get_size (&self) -> f32 {
                $size
            }

            fn get_probability (&self) -> Option<f32> {
                $prob
            }

            fn get_price (&self) -> Option<f32> {
                $price
            }
        }
    };
}

macro_rules! impl_energy {
    ($($target:ident, $size:literal, $energy:literal),+) => {
        $(
            impl_energy!($target, $size, None, None, $energy);
        )*
    };

    ($target:ident, $size:literal, $prob:expr, $price:expr, $energy:literal) => {
        impl_type!($target, $size, $prob, $price);

        impl<'a> EnergySource<'a> for $target {
            fn get_energy (&self) -> u8 {
                $energy
            }
        }
    };
}

// DEFINITIONS
impl_type!(0.1, Iron, Gold, Sulfur, Potassium, Oxygen);
impl_type!(1., Diamond);

impl_energy!(
    Coal, 1., 1,
    Petroleum, 0.2, 3,
    Uranium, 0.1, 8
);

