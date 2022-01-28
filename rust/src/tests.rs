#[test]
fn simulate_system () {
    let mut system : PlanetSystem = Gaussian::new().sample(&mut thread_rng());
    loop {
        system.simulate(Duration::from_weeks(4u32));
        let positions : Vec<f64> = system.get_planets().iter()
            .map(|x| x.velocity.norm())
            .collect();

        println!("{:?}", positions);
        sleep(Duration::from_millis(100));
    }
}