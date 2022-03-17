use std::{sync::Arc, thread, time::Duration};
use llml::vec::EucVecf2;
use slg::{renderer::opengl::OpenGl, Renderer, RenderInstance, generics::Color};

pub mod connection;

fn main() {
    let ogl = Arc::new(OpenGl::new().unwrap());
    let window = ogl.create_instance("Websocket testing", 900u32, 900u32).unwrap();
    
    let mut window = window.write().unwrap();
    let local_player = window.create_circle(EucVecf2::new([0.0, 0.0]), 0.5, Color::WHITE).unwrap();
    let remote_player = window.create_circle(EucVecf2::new([0.5, -0.5]), 0.5, Color::new(128, 128, 255)).unwrap();
    drop(window);

    thread::spawn(move || {
        loop {
            // TODO
            thread::sleep(Duration::from_millis(17));
        }
    });

    ogl.listen_events().unwrap()
}
