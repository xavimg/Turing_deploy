use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct PlayerStats {
    pub level: u8,
    pub max_speed: f64,
    pub max_health: u8
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self { level: 1, max_speed: 10., max_health: 100 }
    }
}