use slg::{Threadly, generics::{Circle, Color}, renderer::opengl::{OpenGl, GlInstance}, RenderInstance};
use crate::{local::PlayerLocation, world_to_local};

#[derive(Clone)]
pub struct RemotePlayer {
    pub location: PlayerLocation,
    pub circle: Threadly<Circle<OpenGl>>
}

impl RemotePlayer {
    #[inline]
    pub fn new (location: PlayerLocation, color: Color, window: Threadly<GlInstance>) -> Self {
        let mut window = window.write().unwrap();
        let circle = window.create_circle(world_to_local(location.position), 0.01, color).unwrap();
        Self { location, circle }
    }
}