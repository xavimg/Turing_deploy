#[test]
fn acc () {
    let sun = Arc::new(Mutex::new(Star::new(
        Color::BLUE,
        0.,
        1048.,
        EucVecd2::default(),
        EucVecd2::default()
    )));

    let earth = Arc::new(Mutex::new(Planet::new(
        Color::GREEN, 0.003146, 
        EucVecd2::new([1., 0.]), 
        EucVecd2::new([0., 1.99e-7])
    )));

    const DT : Duration = Duration::from_secs(604800); // 1 week
    let thread_sun = sun.clone();
    let thread_earth = earth.clone();

    // Printter
    thread::spawn(move || {
        loop {
            let sun1 = thread_sun.lock().unwrap();
            let earth1 = thread_earth.lock().unwrap();

            println!("{:?} \n {:?} \n", sun1.get_pos(), earth1.get_pos());
            drop(sun1);
            drop(earth1);
            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        let sun1 = sun.lock().unwrap();
        let mut earth1 = earth.lock().unwrap();

        let (acc, dir) = earth1.calc_acc(sun1.deref());
        let (acc_earth, _) = acc.unzip();

        earth1.accelerate_and_travel(dir * acc_earth, DT);
        //sun1.accelerate_and_travel(-dir * acc_sun, DT);
    }
}