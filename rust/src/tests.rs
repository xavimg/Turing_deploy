use std::sync::Arc;

#[test]
fn simulate_system () {
    use std::{time::Duration, thread::sleep};
    use rand::{thread_rng, prelude::Distribution};
    use slg::{renderer::opengl::OpenGl, Renderer};
    use crate::{PlanetSystem, Gaussian, Durationx};

    let mut system : PlanetSystem = Gaussian::new().sample(&mut thread_rng());
    let mut ogl = Arc::new(OpenGl::new().unwrap());
    let window = ogl.create_instance("Testing planet factory", 900u32, 900u32).unwrap();

    loop {
        system.simulate(Duration::from_weeks(4u32));
        let positions : Vec<f64> = system.get_planets().iter()
            .map(|x| x.velocity.norm())
            .collect();

        println!("{:?}", positions);
        sleep(Duration::from_millis(100));
    }
}