use std::{sync::{Arc, Mutex}, time::Duration};
use llml::vec::EucVecf2;
use crate::{Uniform, Uniformable, generics::{Color, Circle, RenderElement}};

pub trait Renderer where Self: Sized {
    type Error;
    type Instance: RenderInstance<Self>;
    type Shader: RenderShader<Self>; 
    type Uniform: Uniform<Self>;

    fn new () -> Result<Self, Self::Error>;
    fn create_instance (self: &Arc<Self>, title: impl Into<String>, width: impl Into<u32>, height: impl Into<u32>) -> Result<Arc<Mutex<Self::Instance>>, Self::Error>;
    fn create_shader (&self, code: &str) -> Result<Arc<Self::Shader>, Self::Error>;
    fn listen_events (&self) -> Result<(), Self::Error>;
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

    fn get_children (&self) -> &Vec<Arc<Mutex<dyn RenderElement<R>>>>;
    fn create_circle (&mut self, at: EucVecf2, radius: f32, color: Color) -> Result<Arc<Mutex<Circle<R>>>, R::Error>;
}

pub trait RenderShader<R: Renderer> {
    fn set_uniform<T: Uniformable> (self: &Arc<Self>, key: impl Into<String>, value: &T) -> Result<(), R::Error>;
    fn draw (&self) -> Result<(), R::Error>;

    fn bind (&self) -> Result<(), R::Error>;
    fn unbind (&self) -> Result<(), R::Error>;
}