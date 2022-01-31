#![feature(once_cell, generic_associated_types, result_flattening)]
use std::sync::{Arc, RwLock};

macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )*
    };
}

pub type Threadly<T> = Arc<RwLock<T>>;

pub mod renderer;
pub mod generics;

flat_mod!(traits, uniform);