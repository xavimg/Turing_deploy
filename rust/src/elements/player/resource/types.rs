use crate::Resource;
use serde::{Serialize, Deserialize, Serializer, Deserializer, de::{Visitor, Unexpected}};

macro_rules! impl_type { 
    ($size:literal, $($target:ident),+) => {
        $(
            impl_type!($target, $size, None, None);
        )*
    };

    ($target:ident, $size:literal, $prob:expr, $price:expr) => {
        #[derive(Debug)]
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

        impl Serialize for $target {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                serializer.serialize_str(stringify!($target))
            }
        }

        impl<'a> Deserialize<'a> for $target {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
                struct LocalVisitor;
                impl Visitor<'_> for LocalVisitor {
                    type Value = $target;
        
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result { 
                        formatter.write_str(concat!("A string with the value '", stringify!($target), "'"))
                    }
        
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error, {
                        if v != stringify!($target) {
                            return Err(serde::de::Error::invalid_value(Unexpected::Str(v), &stringify!($target)))
                        }
        
                        Ok($target)
                    }
                }
        
                deserializer.deserialize_str(LocalVisitor)
            }
        }
    };
}

// DEFINITIONS
impl_type!(0.1, Iron, Gold, Sulfur, Potassium, Oxygen, Uranium);
impl_type!(0.2, Petroleum);
impl_type!(1., Diamond, Coal, Bandage);
impl_type!(1.5, Medicine);
impl_type!(2., HeatlthKit);
impl_type!(5., Railgun);