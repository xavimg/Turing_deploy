use std::{num::NonZeroU32, sync::{Arc, Mutex}};
use gl33::{global_loader::{glDeleteProgram, glDeleteShader, glUseProgram, glBindVertexArray, glDrawArrays}, GL_TRIANGLES};
use crate::{RenderShader, UniformMap, Uniform};
use super::{OpenGl, GlUniform};

macro_rules! insert_uniforms {
    ($result:expr, $uniforms:expr, $($key:literal),+) => {
        $(
            $uniforms.insert($key.to_string(), GlUniform::new($result.clone(), $key)?);
        )*
    };
}

pub struct GlShader {
    pub(super) program: NonZeroU32,
    pub(super) vertex: NonZeroU32,
    pub(super) fragment: NonZeroU32,
    uniforms: Mutex<UniformMap<OpenGl>>
}

impl GlShader {
    pub fn new (program: NonZeroU32, vertex: NonZeroU32, fragment: NonZeroU32) -> Result<Arc<Self>, String> {
        let result = Arc::new(GlShader {
            program,
            vertex,
            fragment,
            uniforms: Mutex::new(UniformMap::<OpenGl>::with_capacity(4)) // it's safe to assume there will be at least one extra uniform
        });

        let mut uniforms = result.uniforms.lock().map_err(|e| e.to_string())?;
        insert_uniforms!(result, uniforms, "aspectRatio", "selfPosition", "selfScale", "color");
        
        drop(uniforms);
        Ok(result)
    }
}

impl RenderShader<OpenGl> for GlShader {
    fn set_uniform<T: crate::Uniformable> (self: &Arc<Self>, key: impl Into<String>, value: &T) -> Result<(), String> {
        let mut lock = self.uniforms.lock().map_err(|e| e.to_string())?;
        let key = key.into();

        match lock.get_mut(&key) {
            Some(uniform) => {
                uniform.set_value(value)
            },

            None => {
                let mut uniform = GlUniform::new(self.clone(), key.clone())?;
                uniform.set_value(value)?;
                lock.insert(key, uniform);
                Ok(())
            }
        }
    }
    
    fn bind (&self) -> Result<(), String> {
        glUseProgram(self.program.into());
        Ok(())
    }

    fn unbind (&self) -> Result<(), String> {
        glUseProgram(0);
        Ok(())
    }

    fn draw (&self) -> Result<(), String> {
        self.bind()?;

        glBindVertexArray(OpenGl::FILLER_MESH.get());
        unsafe { glDrawArrays(GL_TRIANGLES, 0, 6); }
        glBindVertexArray(0);

        Ok(())
    }
}

impl Drop for GlShader {
    fn drop (&mut self) {
        glDeleteProgram(self.program.into());
        glDeleteShader(self.vertex.into());
        glDeleteShader(self.fragment.into());
    }
}