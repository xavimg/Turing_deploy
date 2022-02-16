use std::{sync::{Arc}, time::Duration};
use llml::vec::{EucVecf2};
use slg::RenderInstance;
use crate::{Color, create_system, Durationx};

pub async fn simulate_system () {
    use slg::{renderer::opengl::OpenGl, Renderer};
    use crate::{PlanetSystem};

    let mut system : PlanetSystem = create_system().await;
    let max_dist = system.planets.last().unwrap().position.norm() * 2.;
    let ogl = Arc::new(OpenGl::new().unwrap());
    
    let window = ogl.create_instance("Testing planet factory", 900u32, 900u32).unwrap();
    let mut window = window.write().unwrap();
    let _ = window.create_circle(EucVecf2::default(), 0.025, Color::WHITE.into()).unwrap();

    let mut planets = Vec::with_capacity(system.planets.len());
    for planet in system.planets.iter() {
        let circle = window.create_circle((planet.position / max_dist).into(), 0.0125, Color::RED.into()).unwrap();
        planets.push(circle);
    }

    drop(window);
    tokio::spawn(async move {
        loop {
            system.simulate(Duration::from_months(3u64)).await;

            for i in 0..planets.len() {
                planets[i].write().unwrap().position = (system.planets[i].position / max_dist).into();
            }

            tokio::time::sleep(Duration::from_millis(17)).await;
        }
    });

    ogl.listen_events().unwrap();
}