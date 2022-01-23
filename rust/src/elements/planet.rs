use llml::vec::EucVecd3;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Planet {
    mass: f64,
    position: EucVecd3,
    velocity: EucVecd3
}