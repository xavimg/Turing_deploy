use randerive::Rand;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Hash, PartialEq, Eq, Deserialize, Rand)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub mod color_rgba {
    use serde::{Serializer, Deserializer, Deserialize};
    use super::Color;

    #[inline]
    pub fn serialize<S: Serializer> (color: &Color, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(color.as_u32())
    }

    #[inline]
    pub fn deserialize<'de, D: Deserializer<'de>> (deserializer: D) -> Result<Color, D::Error> {
        u32::deserialize(deserializer).map(|x| x.into())
    }
}

impl Color {
    pub const WHITE : Color = Color::new(255, 255, 255);
    pub const BLACK : Color = Color::new(0, 0, 0);

    pub const RED : Color = Color::new(255, 0, 0);
    pub const GREEN : Color = Color::new(0, 255, 0);
    pub const BLUE : Color = Color::new(0, 0, 255);

    pub const fn new (r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub const fn from_f32 (r: f32, g: f32, b: f32) -> Color {
        Color::new(
            (r * 255.) as u8, 
            (g * 255.) as u8,
            (b * 255.) as u8
        )
    }

    pub const fn from_f64 (r: f64, g: f64, b: f64) -> Color {
        Color::new(
            (r * 255.) as u8,
            (g * 255.) as u8,
            (b * 255.) as u8
        )
    }

    #[inline]
    pub fn as_u32 (&self) -> u32 {
        0xff000000 | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl Into<u32> for Color {
    #[inline]
    fn into(self) -> u32 {
        self.as_u32()
    }
}

impl From<u32> for Color {
    #[inline]
    fn from(x: u32) -> Self {
        debug_assert_eq!(x >> 24, 255);
        let b = x as u8;
        let g = (x >> 8) as u8;
        let r = (x >> 16) as u8;
        Self { r, g, b }
    }
}