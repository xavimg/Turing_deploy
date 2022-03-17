use slg::{Threadly, generics::Circle, renderer::opengl::OpenGl};
use crate::local::PlayerLocation;

pub struct RemotePlayer {
    location: PlayerLocation,
    token: String,
    circle: Threadly<Circle<OpenGl>>
}