use std::{collections::HashMap};
use lazy_static::{lazy_static};
use serde::{Serialize, Deserialize};
use strum::EnumIter;

#[macro_export]
macro_rules! map {
    {$($key:expr => $value:expr),+} => {
        { 
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map 
        }
    };
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
pub enum Resource {
    Iron, Gold, Sulfur, Potassium, Nitrogen, Oxygen, Diamond,
    Coal, Petroleum, Uranium,
    Bandage, Medicine, HealthKit,
    Railgun,
    Bullet, Niter, Gunpowder   
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceType {
    EnergySource { energy: u8 },
    Healer { health: u8 },
    Damager { damage: u8, require: (Resource, u8) },
    Composite(HashMap<Resource, u8>)
}

lazy_static! {
    pub static ref BULLET_COMPOSITE : HashMap<Resource, u8> = map! {
        Resource::Iron => 1
    };

    pub static ref NITER_COMPOSITE : HashMap<Resource, u8> = map! {
        Resource::Potassium => 1,
        Resource::Nitrogen => 1,
        Resource::Oxygen => 3
    };

    pub static ref GUNPOWDER_COMPOSITE : HashMap<Resource, u8> = map! {
        Resource::Sulfur => 1,
        Resource::Coal => 1,
        Resource::Niter => 1
    };
}

impl Resource {
    pub fn get_size (&self) -> f32 {
        match self.get_type() {
            Some(ResourceType::Composite(map)) => {
                map.into_iter()
                    .map(|(k, v)| k.get_size() * (v as f32))
                    .sum()
            }

            _ => match self {
                Self::Railgun => 5.,
                Self::HealthKit => 2.,
                Self::Medicine => 1.5,
                Self::Diamond | Self::Coal | Self::Bandage => 1.,
                Self::Petroleum => 0.2,
                _ => 0.1
            }
        }
    }

    pub fn get_type (&self) -> Option<ResourceType> {
        match self {
            Self::Coal => Some(ResourceType::EnergySource { energy: 1 }),
            Self::Petroleum => Some(ResourceType::EnergySource { energy: 3 }),
            Self::Uranium => Some(ResourceType::EnergySource { energy: 8 }),

            Self::Bandage => Some(ResourceType::Healer { health: 5 }),
            Self::Medicine => Some(ResourceType::Healer { health: 15 }),
            Self::HealthKit => Some(ResourceType::Healer { health: 30 }),

            Self::Railgun => Some(ResourceType::Damager { damage: 30, require: (Resource::Bullet, 1) }),
            Self::Bullet => Some(ResourceType::Composite(map!{ Resource::Iron => 1 })),
            Self::Niter => Some(ResourceType::Composite(map!{ Resource::Potassium => 1, Resource::Nitrogen => 1, Resource::Oxygen => 3 })),
            Self::Gunpowder => Some(ResourceType::Composite(map!{ Resource::Sulfur => 1, Resource::Coal => 1, Resource::Niter => 1 })),

            _ => None
        }
    }

    pub fn is_composite (&self) -> bool {
        match self {
            Self::Bullet | Self::Niter | Self::Gunpowder => true,
            _ => false
        }
    }
}