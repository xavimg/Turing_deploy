use slg::{renderer::opengl::OpenGl, Renderer};

#[test]
fn init () {
    match init_gl() {
        Err(x) => panic!("{x}"),
        Ok(_) => {}
    }
}

#[inline(always)]
fn init_gl () -> Result<(), String> {
    let mut ogl = OpenGl::new()?;
    let window = ogl.create_instance("Hello world", 900u32, 900u32)?;
    ogl.listen_events()
}