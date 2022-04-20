use std::sync::Arc;
use llml::vec::EucVecf2;
use crate::{Renderer, RenderShader};
use super::Color;

pub trait RenderElement<R: Renderer>: Send + Sync {
    fn get_shader (&self) -> &Arc<R::Shader>;
    fn get_shader_mut (&mut self) -> &mut Arc<R::Shader>;

    /// Return position as in center of the element ```(0,0)```
    fn get_position (&self) -> EucVecf2;

    /// Return distance from center of the element to it's extremity
    fn get_size (&self) -> EucVecf2;
    fn get_color (&self) -> &Color;

    fn render (&self) -> Result<(), String> {
        let shader = self.get_shader();
        shader.bind()?;
        shader.set_uniform("aspectRatio", &1f32)?; // TODO
        shader.set_uniform("selfPosition", &self.get_position())?;
        shader.set_uniform("selfScale", &self.get_size())?;
        shader.set_uniform("color", self.get_color())?;
        shader.draw()
    }
}

// CIRCLE
pub struct Circle<R: Renderer> {
    shader: Arc<R::Shader>,
    pub position: EucVecf2,
    pub color: Color,
    pub radius: f32
}

unsafe impl<R: Renderer> Send for Circle<R> {}
unsafe impl<R: Renderer> Sync for Circle<R> {}

impl<R: Renderer> Circle<R> {
    pub fn new (shader: Arc<R::Shader>, position: EucVecf2, radius: f32, color: Color) -> Self {
        Self {
            shader,
            position,
            color,
            radius
        }
    }
}

impl<R: Renderer> RenderElement<R> for Circle<R> where Self: Send + Sync {
    fn get_shader (&self) -> &Arc<R::Shader> {
        &self.shader
    }

    fn get_shader_mut (&mut self) -> &mut Arc<R::Shader> {
        &mut self.shader
    }

    fn get_position (&self) -> EucVecf2 {
        self.position
    }

    fn get_size (&self) -> EucVecf2 {
        EucVecf2::from_scal(self.radius)
    }

    fn get_color (&self) -> &Color {
        &self.color
    }
}