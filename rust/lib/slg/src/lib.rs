#![feature(once_cell, generic_associated_types, result_flattening)]

macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )*
    };
}

pub mod renderer;
pub mod generics;

flat_mod!(traits, uniform);