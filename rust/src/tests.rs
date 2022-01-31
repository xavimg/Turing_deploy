use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};
use llml::vec::{EucVecf2};
use slg::RenderInstance;
use crate::{Color, Durationx};

/*
pub fn simulate_system () {
    use rand::{thread_rng, prelude::Distribution};
    use slg::{renderer::opengl::OpenGl, Renderer};
    use crate::{PlanetSystem, Gaussian};

    let system : PlanetSystem = Gaussian::new().sample(&mut thread_rng());
    let max_dist = system.get_planets()[0].position.norm();
    let ogl = Arc::new(OpenGl::new().unwrap());
    
    let window = ogl.create_instance("Testing planet factory", 900u32, 900u32).unwrap();
    let mut window = window.lock().unwrap();

    let _ = window.create_circle(EucVecf2::default(), 0.25, Color::BLACK.into()).unwrap();
    let planet_circle = window.create_circle(system.get_planets()[0].position.unit().into(), 0.125, Color::RED.into()).unwrap();

    let system = Arc::new(Mutex::new(system));
    thread::spawn(move || {
        loop {
            let mut lock = system.lock().unwrap();
            lock.simulate(Duration::from_months(1u32));
            planet_circle.lock().unwrap().position = (lock.get_planets()[0].position / max_dist).into();
    
            drop(lock);
            sleep(Duration::from_millis(17));
        }
    });

    ogl.listen_events();
}*/