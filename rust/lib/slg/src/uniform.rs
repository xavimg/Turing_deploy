use std::{collections::{HashMap}, sync::{Arc}};
use llml::vec::{EucVecf2, EucVecf3, EucVecf4, EucVecd2, EucVecd3, EucVecd4};
use crate::Renderer;

pub type UniformMap<R: Renderer> = HashMap<String, R::Uniform>;

pub trait Uniform<R: Renderer> {
    fn new (parent: Arc<R::Shader>, key: impl Into<String>) -> Result<Self, String> where Self: Sized;

    fn get_parent (&self) -> Arc<R::Shader>;
    fn get_id (&self) -> u32;
    fn get_key (&self) -> &String;
    fn set_value<T: Uniformable> (&mut self, value: &T) -> Result<(), String> where Self: Sized {
        value.set_value(self)
    }

    fn set_int (&mut self, value: i32) -> Result<(), String>;
    fn set_uint (&mut self, value: u32) -> Result<(), String>;
    fn set_float (&mut self, value: f32) -> Result<(), String>;
    fn set_double (&mut self, value: f64) -> Result<(), String>;

    fn set_vec2f (&mut self, value: EucVecf2) -> Result<(), String>;
    fn set_vec3f (&mut self, value: EucVecf3) -> Result<(), String>;
    fn set_vec4f (&mut self, value: EucVecf4) -> Result<(), String>;

    fn set_vec2d (&mut self, value: EucVecd2) -> Result<(), String>;
    fn set_vec3d (&mut self, value: EucVecd3) -> Result<(), String>;
    fn set_vec4d (&mut self, value: EucVecd4) -> Result<(), String>;
}

pub trait Uniformable {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String>; 
}

impl Uniformable for i32 {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String> {
        target.set_int(*self)
    }
}

impl Uniformable for u32 {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String> {
        target.set_uint(*self)
    }
}

impl Uniformable for f32 {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String> {
        target.set_float(*self)
    }
}

impl Uniformable for f64 {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String> {
        target.set_double(*self)
    }
}

impl Uniformable for EucVecf2 {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String> {
        target.set_vec2f(*self)
    }
}

impl Uniformable for EucVecf3 {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String> {
        target.set_vec3f(*self)
    }
}

impl Uniformable for EucVecf4 {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String> {
        target.set_vec4f(*self)
    }
}

impl Uniformable for EucVecd2 {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String> {
        target.set_vec2d(*self)
    }
}

impl Uniformable for EucVecd3 {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String> {
        target.set_vec3d(*self)
    }
}

impl Uniformable for EucVecd4 {
    fn set_value<R: Renderer, U: Uniform<R>> (&self, target: &mut U) -> Result<(), String> {
        target.set_vec4d(*self)
    }
}