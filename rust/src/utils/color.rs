use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
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
}