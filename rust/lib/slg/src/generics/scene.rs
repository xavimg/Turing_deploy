use std::{time::Duration, sync::{Arc, RwLockWriteGuard}, marker::PhantomData, thread::{self, JoinHandle, sleep}, borrow::BorrowMut};
use llml::vec::EucVecf2;

use crate::{Threadly, Renderer, RenderInstance};
use super::{RenderElement, Color, Circle};

// STRUCT
pub struct SceneBuilder<'a, R: Renderer, T> {
    pub instance: Threadly<R::Instance>,
    data: Vec<Threadly<dyn RenderElement<R>>>,
    label: PhantomData<&'a T>
}

pub struct Scene<'a, R: Renderer, T, F: FnMut(&Duration, &mut T)> {
    pub instance: Threadly<R::Instance>,
    data: Vec<Threadly<dyn RenderElement<R>>>,
    on_update: (Duration, F),
    label: PhantomData<&'a T>
}

// IMPL
impl<'a, R: Renderer> SceneBuilder<'a, R, ()> {
    pub fn new (parent: &Arc<R>, title: impl Into<String>, width: u32, height: u32) -> Result<Self, String> {
        let instance = parent.create_instance(title, width, height)?;
        Ok(Self { 
            instance,
            data: Vec::with_capacity(1),
            label: PhantomData
        })
    }
}

impl<'a, R: Renderer, T> SceneBuilder<'a, R, T> {
    pub fn add_data<U: 'static + RenderElement<R>> (mut self, data: Threadly<U>) -> SceneBuilder<'a, R, (T, RwLockWriteGuard<'a,U>)> {
        self.data.push(data);
        SceneBuilder {
            instance: self.instance,
            data: self.data,
            label: PhantomData
        }
    }

    pub fn add_circle (self, at: EucVecf2, radius: f32, color: Color) -> Result<SceneBuilder<'a, R, (T, RwLockWriteGuard<'a, Circle<R>>)>, String> where Circle<R>: Send + Sync, R: 'static {
        let circle = self.instance.write().map_err(|e| e.to_string())?.create_circle(at, radius, color)?;
        Ok(self.add_data(circle))
    }

    pub fn build<F: FnMut(&Duration, &mut T)> (self, interval: Duration, update: F) -> Scene<'a, R, T, F> {
        Scene { 
            instance: self.instance, 
            data: self.data, 
            on_update: (interval, update), 
            label: self.label
        }
    }
}

impl<'a, R: Renderer, A, B, C> SceneBuilder<'a, R, ((A,B),C)> {
    pub fn flatten (self) -> SceneBuilder<'a, R, (A,B,C)> {
        SceneBuilder {
            instance: self.instance,
            data: self.data,
            label: PhantomData
        }
    }

    pub fn flatten_first (self) -> SceneBuilder<'a, R, (B,C)> {
        SceneBuilder {
            instance: self.instance,
            data: self.data,
            label: PhantomData
        }
    }
}

impl<'a, R: 'static + Renderer, T, F: 'static + FnMut(&Duration, &mut T) + Send> Scene<'a, R, T, F> {
    pub fn start (mut self) {
        thread::spawn(move || {
            loop {
                let mut refs = Vec::with_capacity(self.data.len());
                for elem in &self.data {
                    refs.push(elem.write().unwrap())
                }
                
                println!("{:?}", refs.len());
                let casted = unsafe { &mut *(refs.as_mut_ptr() as *mut T) };
                (self.on_update.1)(&self.on_update.0, casted);

                drop(refs);
                sleep(self.on_update.0.clone())
            }
        });
    }
}