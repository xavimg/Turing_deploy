use std::{lazy::Lazy, num::NonZeroU32, sync::{Arc, RwLock, atomic::{AtomicBool, Ordering}}, intrinsics::transmute};
use glutin::{window::Window, ContextWrapper, PossiblyCurrent};
use llml::vec::EucVecf2;
use crate::{RenderInstance, generics::{Color, Circle, RenderElement}, Threadly, Renderer};
use super::{OpenGl};

pub struct GlInstance {
    pub(super) context: ContextWrapper<PossiblyCurrent, Window>,
    pub(super) title: String,
    pub(super) keyboard: [AtomicBool; 161],
    children: Vec<Threadly<dyn RenderElement<OpenGl>>>
}

impl GlInstance {
    pub fn new (title: String, context: ContextWrapper<PossiblyCurrent, Window>) -> Result<Self, String> {
        Lazy::<NonZeroU32>::force(&OpenGl::FILLER_MESH);
        Ok(Self {
            context,
            title,
            children: Vec::with_capacity(1),
            keyboard: unsafe { transmute([false; 161]) }
        })
    }
}

unsafe impl Send for GlInstance {}
unsafe impl Sync for GlInstance {}

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

    fn get_children(&self) -> &Vec<Threadly<dyn RenderElement<OpenGl>>> {
        &self.children
    }

    fn create_circle (&mut self, at: EucVecf2, radius: f32, color: Color) -> Result<Threadly<Circle<OpenGl>>, String> {
        let shader = OpenGl::create_shader(include_str!("glsl/circle.frag"))?;
        let circle = Arc::new(RwLock::new(Circle::<OpenGl>::new(shader, at, radius, color)));

        self.children.push(circle.clone());
        Ok(circle)
    }

    #[inline]
    fn is_pressed(&self, key: crate::generics::KeyboardKey) -> bool {
        self.keyboard[usize::from(key)].load(Ordering::Relaxed)
    }
}