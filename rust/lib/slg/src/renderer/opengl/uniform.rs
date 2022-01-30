use std::{sync::{Arc}, time::Duration};
use gl33::global_loader::{glGetUniformLocation, glUniform1i, glUniform1ui, glUniform1f, glUniform2fv, glUniform3fv, glUniform4fv};
use crate::{Uniform, RenderShader};
use super::{OpenGl, GlShader};

pub struct GlUniform {
    parent: Arc<GlShader>,
    id: i32,
    key: String
}

impl Uniform<OpenGl> for GlUniform {
    fn new (parent: Arc<GlShader>, key: impl Into<String>) -> Result<Self, <OpenGl as crate::Renderer>::Error> where Self: Sized {
        let key = key.into();

        parent.bind()?;
        let id = unsafe { glGetUniformLocation(parent.program.get(), format!("{key}\0").as_ptr()) };
        if id < 0 { return Err(format!("Could not find uniform '{key}'")); }

        Ok(Self {
            parent,
            id,
            key
        })
    }

    fn get_parent (&self) -> Arc<<OpenGl as crate::Renderer>::Shader> {
        self.parent.clone()
    }

    fn get_id (&self) -> u32 {
        self.id as u32
    }

    fn get_key (&self) -> &String {
        &self.key
    }

    fn set_int (&mut self, value: i32) -> Result<(), <OpenGl as crate::Renderer>::Error> {
        self.parent.bind()?;
        unsafe { glUniform1i(self.id, value); }
        Ok(())
    }

    fn set_uint (&mut self, value: u32) -> Result<(), <OpenGl as crate::Renderer>::Error> {
        self.parent.bind()?;
        unsafe { glUniform1ui(self.id, value); }
        Ok(())
    }

    fn set_float (&mut self, value: f32) -> Result<(), <OpenGl as crate::Renderer>::Error> {
        self.parent.bind()?;
        unsafe { glUniform1f(self.id, value); }
        Ok(())
    }

    fn set_double (&mut self, value: f64) -> Result<(), <OpenGl as crate::Renderer>::Error> {
        Err("Double precision unavailable".to_string())
    }

    fn set_vec2f (&mut self, value: llml::vec::EucVecf2) -> Result<(), <OpenGl as crate::Renderer>::Error> {
        self.parent.bind()?;
        let array = &Into::<[f32;2]>::into(value);
        unsafe { glUniform2fv(self.id, 1, array.as_ptr().cast()); }
        Ok(())
    }

    fn set_vec3f (&mut self, value: llml::vec::EucVecf3) -> Result<(), <OpenGl as crate::Renderer>::Error> {
        self.parent.bind()?;
        let array = &Into::<[f32;3]>::into(value);
        unsafe { glUniform3fv(self.id, 1, array.as_ptr().cast()); }
        Ok(())
    }

    fn set_vec4f (&mut self, value: llml::vec::EucVecf4) -> Result<(), <OpenGl as crate::Renderer>::Error> {
        self.parent.bind()?;
        let array = &Into::<[f32;4]>::into(value);
        unsafe { glUniform4fv(self.id, 1, array.as_ptr().cast()); }
        Ok(())
    }

    fn set_vec2d (&mut self, value: llml::vec::EucVecd2) -> Result<(), <OpenGl as crate::Renderer>::Error> {
        Err("Double precision unavailable".to_string())
    }

    fn set_vec3d (&mut self, value: llml::vec::EucVecd3) -> Result<(), <OpenGl as crate::Renderer>::Error> {
        Err("Double precision unavailable".to_string())
    }

    fn set_vec4d (&mut self, value: llml::vec::EucVecd4) -> Result<(), <OpenGl as crate::Renderer>::Error> {
        Err("Double precision unavailable".to_string())
    }
}