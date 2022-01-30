use std::{sync::Arc, thread, time::Duration};
use llml::vec::EucVecf2;
use slg::{renderer::opengl::OpenGl, Renderer, RenderInstance, generics::Color};

fn main () {
    match init_gl() {
        Err(x) => panic!("{x}"),
        Ok(_) => {}
    }
}

fn init_gl () -> Result<(), String> {
    let ogl = Arc::new(OpenGl::new()?);
    let window = ogl.create_instance("Hello world", 900u32, 900u32)?;
    let mut window = window.lock().map_err(|e| e.to_string())?;
    
    let c1 = window.create_circle(EucVecf2::default(), 0.5, Color::new(255, 128, 128))?;
    let c2 = window.create_circle(EucVecf2::new([0.5, -0.5]), 0.25, Color::new(255, 0, 0))?;
    drop(window);

    thread::spawn(move || {
        loop {
            let mut c1 = c1.lock().unwrap();
            let mut c2 = c2.lock().unwrap();

            c1.position += EucVecf2::new([0.001, 0.]);
            c2.position += EucVecf2::new([0., 0.001]);

            drop(c1);
            drop(c2);
            thread::sleep(Duration::from_millis(100))
        }
    });

    ogl.listen_events()
}