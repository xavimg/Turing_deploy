use std::{sync::{Arc, Mutex}};
use llml::vec::EucVecf2;
use crate::{Uniform, Uniformable, generics::{Color, Circle, RenderElement}, Threadly};

pub trait Renderer where Self: Sized {
    type Instance: RenderInstance<Self>;
    type Shader: RenderShader<Self>; 
    type Uniform: Uniform<Self>;

    fn new () -> Result<Self, String>;
    fn create_instance (self: &Arc<Self>, title: impl Into<String>, width: impl Into<u32>, height: impl Into<u32>) -> Result<Threadly<Self::Instance>, String>;
    fn create_shader (&self, code: &str) -> Result<Arc<Self::Shader>, String>;
    fn listen_events (&self) -> Result<(), String>;
}

pub trait RenderInstance<R: Renderer> {
    fn get_title (&self) -> &String;
    fn get_width (&self) -> u32;
    fn get_height (&self) -> u32;
    
    fn get_size (&self) -> (u32, u32) {
        (self.get_width(), self.get_height())
    }

    fn get_aspect_ratio (&self) -> f32 {
        let (width, height) = self.get_size();
        (width as f32) / (height as f32)
    }

    fn get_children (&self) -> &Vec<Threadly<dyn RenderElement<R>>>;
    fn create_circle (&mut self, at: EucVecf2, radius: f32, color: Color) -> Result<Threadly<Circle<R>>, String>;
}

pub trait RenderShader<R: Renderer> {
    fn set_uniform<T: Uniformable> (self: &Arc<Self>, key: impl Into<String>, value: &T) -> Result<(), String>;
    fn draw (&self) -> Result<(), String>;

    fn bind (&self) -> Result<(), String>;
    fn unbind (&self) -> Result<(), String>;
}