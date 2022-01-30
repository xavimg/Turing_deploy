use std::{lazy::Lazy, num::NonZeroU32, sync::{Arc, Mutex}};
use glutin::{window::Window, ContextWrapper, PossiblyCurrent};
use llml::vec::EucVecf2;
use crate::{RenderInstance, generics::{Color, Circle, RenderElement}, Renderer};
use super::{OpenGl};

pub struct GlInstance {
    pub(super) context: ContextWrapper<PossiblyCurrent, Window>,
    pub(super) title: String,
    parent: Arc<OpenGl>,
    children: Vec<Arc<Mutex<dyn RenderElement<OpenGl>>>>
}

impl GlInstance {
    pub fn new (parent: Arc<OpenGl>, title: String, context: ContextWrapper<PossiblyCurrent, Window>) -> Result<Self, String> {
        Lazy::<NonZeroU32>::force(&OpenGl::FILLER_MESH);
        Ok(Self {
            parent,
            context,
            title,
            children: Vec::with_capacity(1)
        })
    }
}

impl RenderInstance<OpenGl> for GlInstance {
    fn get_title (&self) -> &String {
        &self.title
    }

    fn get_width (&self) -> u32 {
        self.context.window().inner_size().width
    }

    fn get_height (&self) -> u32 {
        self.context.window().inner_size().height
    }

    fn get_size(&self) -> (u32, u32) {
        self.context.window().inner_size().into()
    }

    fn get_children(&self) -> &Vec<Arc<Mutex<dyn RenderElement<OpenGl>>>> {
        &self.children
    }

    fn create_circle (&mut self, at: EucVecf2, radius: f32, color: Color) -> Result<Arc<Mutex<Circle<OpenGl>>>, String> {
        let shader = self.parent.create_shader(include_str!("glsl/circle.frag"))?;
        let circle = Arc::new(Mutex::new(Circle::<OpenGl>::new(shader, at, radius, color)));

        self.children.push(circle.clone());
        Ok(circle)
    }
}