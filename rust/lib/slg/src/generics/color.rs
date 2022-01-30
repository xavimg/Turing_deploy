use llml::vec::EucVecf3;
use crate::Uniformable;

#[repr(transparent)]
pub struct Color([f32;3]);

impl Color {
    pub fn new (r: u8, g: u8, b: u8) -> Self {
        let vec = EucVecf3::new([
            r as f32,
            g as f32,
            b as f32
        ]);

        Self((vec / 255.).into())
    }

    pub fn red (&self) -> f32 {
        self.0[0]
    }

    pub fn green (&self) -> f32 {
        self.0[1]
    }

    pub fn blue (&self) -> f32 {
        self.0[2]
    }
}

impl Uniformable for Color {
    fn set_value<R: crate::Renderer, U: crate::Uniform<R>> (&self, target: &mut U) -> Result<(), R::Error> {
        target.set_vec3f(EucVecf3::new(self.0))
    }
}