use serde::{Serialize, Deserialize};

pub trait Resource<'a>: Serialize + Deserialize<'a> {
    fn get_size (&self) -> f32;
    fn get_probability (&self) -> Option<f32>;
    fn get_price (&self) -> Option<f32>;
}

pub trait EnergySource<'a>: Resource<'a> {
    fn get_energy (&self) -> u8;
}

pub trait Healer<'a>: Resource<'a> {
    fn get_health (&self) -> u8;
}

pub trait Damager<'a>: Resource<'a> {
    fn get_damage (&self) -> u8;
}

flat_mod!(types);