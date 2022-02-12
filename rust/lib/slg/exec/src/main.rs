use std::{sync::Arc, thread};
use llml::vec::EucVecf2;
use slg::{renderer::opengl::OpenGl, Renderer, RenderInstance, generics::{Color, SceneWriter}};
use slg::scene_with;

fn main () {
    match init_gl() {
        Err(x) => panic!("{x}"),
        Ok(_) => {}
    }
}

fn init_gl () -> Result<(), String> {
    let ogl = Arc::new(OpenGl::new()?);
    let window = ogl.create_instance("Hello world", 900u32, 900u32)?;
    let mut window = window.write().map_err(|e| e.to_string())?;
    
    let c1 = window.create_circle(EucVecf2::default(), 0.5, Color::new(255, 128, 128))?;
    let c2 = window.create_circle(EucVecf2::new([0.5, -0.5]), 0.25, Color::new(255, 0, 0))?;
    drop(window);

    thread::spawn(move || {
        loop {
            let mut c1 = c1.write().unwrap();
            let mut c2 = c2.write().unwrap();

            c1.position += EucVecf2::new([f32::EPSILON, 0.]);
            c2.position += EucVecf2::new([0., f32::EPSILON]);

            drop(c1);
            drop(c2);
        }
    });

    ogl.listen_events()
}