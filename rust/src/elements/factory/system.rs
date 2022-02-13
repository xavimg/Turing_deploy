use std::{time::Duration, intrinsics::transmute, cmp::Ordering, rc::Rc, ops::DerefMut};
use crate::{Durationx};
use llml::{vec::EucVecd2, others::Complxd};
use rand::{prelude::{Distribution, ThreadRng}, distributions::{Standard, Uniform}, thread_rng};
use tokio::{task::JoinError};
use crate::{Gaussian, PlanetSystem, Star, Planet, loop_clamp, Color};

const MIN_MASS : f64 = 1.01e-9;
const MAX_MASS : f64 = 0.01146;

/// Creates a planetary system
async fn create_system () -> PlanetSystem {
    let mut thread_rng = thread_rng();
    let gaussian = Gaussian::new();
    let star : Star = gaussian.sample(&mut thread_rng);

    loop {
        //let count = self.sample_with(rng, 0.974172620904933, 3.44274809160305).round();
        let count : f64 = 1.;
        if count < 1. { continue }

        let count = count.round() as usize;
        let mut planets : Vec<Planet> = Vec::with_capacity(count);
        let mut accum_dist = 0.;

        for i in 0..count {
            let mass = loop_clamp(MIN_MASS, MAX_MASS, || gaussian.sample_with(&mut thread_rng, 4.01265200872978, 1.81770640331076));
            let color : Color = Standard.sample(&mut thread_rng);
            
            //let dist : f64 = self.sample(rng);
            let dist = 1f64;
            //let dist = accum_dist + (dist.abs() + 4.5);
            accum_dist += dist;

            let angle = Uniform::new_inclusive(0., std::f64::consts::TAU).sample(&mut thread_rng);
            let position : EucVecd2 = (dist * Complxd::expi(angle)).into();

            planets.insert(i, Planet::new(i, color, mass, position, EucVecd2::default()));
        }

        return PlanetSystem::new(star, planets)
    }
}

/// Genetic algorithm to find optimal initial velocities for planets
struct GeneticSystem {
    systems: Vec<PlanetSystem>,
    mutation_rate: f64,
    rng: Rc<(Standard, std::sync::Mutex<ThreadRng>)>
}

impl GeneticSystem {
    const ZERO : EucVecd2 = unsafe { transmute([0., 0.]) };
    const TIME_LIMIT : Duration = Duration::from_secs(3155760000); // 100 years

    fn new (system: PlanetSystem, len: usize, mutation_rate: f64, mut rng: ThreadRng) -> Self {
        let systems = (0..len).into_iter().map(|_| {
            let mut this = system.clone();
            for planet in this.planets.iter_mut() { planet.velocity = Standard.sample(&mut rng); }
            this
        }).collect();

        Self { systems, mutation_rate, rng: Rc::new((Standard, std::sync::Mutex::new(rng))) }
    }

    async fn fitness (systems: impl IntoIterator<Item = PlanetSystem>, time_step: Duration) -> Result<Vec<(PlanetSystem, f64)>, JoinError> {
        let stream = systems.into_iter()
            .map(|mut system| tokio::spawn(async move {
                let mut time = Duration::ZERO;

                while time < Self::TIME_LIMIT {
                    system.simulate(time_step);
                    if system.planets.iter().any(|planet| planet.position == Self::ZERO || !planet.velocity.x().is_finite() || !planet.velocity.y().is_finite()) { break; }
                    time = time + time_step;
                }

                (system, time.as_weeks() as f64)
            }));

        futures::future::try_join_all(stream).await
    }

    async fn epoch (&mut self, selection_size: Option<usize>, time_step: Duration) -> Result<(), JoinError> {
        // Calculate fitness and sort descending
        let selection_size = selection_size.unwrap_or(self.systems.len());
        let mut fitness = Self::fitness(self.systems.iter().cloned(), time_step).await?;
        fitness.sort_by(|(_, x), (_, y)| y.partial_cmp(x).unwrap_or(Ordering::Equal));

        // Select the most fit systems & calculate their joint fitness
        let selection : &mut [(PlanetSystem, f64)] = &mut fitness[..selection_size];
        let mut sum = 0f64;
        for (_, weight) in selection.iter() { sum += *weight; } 

        // Calculate new planet velocities for each planetary system
        let new_systems = self.systems.iter_mut().for_each(|system| {
            let mut lock = self.rng.1.lock().unwrap();
            // Random Mutation
            if <Standard as Distribution<f64>>::sample(&self.rng.0, lock.deref_mut()) <= self.mutation_rate {
                for planet in system.planets.iter_mut() { planet.velocity = Standard.sample(lock.deref_mut()); }
                return;
            }

            // Select parents for new planet velocities
            let ((father, father_weight), (mother, mother_weight)) = Self::selection_parents(lock, selection, sum);
            let weight_sum = *father_weight + *mother_weight;
            let limit = *father_weight / weight_sum;

            // Assign new velocities
            for i in 0..system.planets.len() {
                let mut lock = self.rng.1.lock().unwrap();
                let rand = <Standard as Distribution<f64>>::sample(&self.rng.0, lock.deref_mut());
                system.planets[i].velocity = if rand <= limit { father.planets[i].velocity } else { mother.planets[i].velocity }
            }
        });

        Ok(())
    }

    async fn train (&mut self, selection_size: impl Into<Option<usize>>, epochs: usize, time_step: Duration) -> Result<(), JoinError> {
        let selection_size = selection_size.into();
        for i in 0..(epochs-1) { self.epoch(selection_size, time_step).await?; }
        self.epoch(selection_size, time_step).await
    }

    fn selection_parents<'a> (rng: impl DerefMut<Target = ThreadRng>, values: &'a [(PlanetSystem, f64)], max: f64) ->(&'a (PlanetSystem, f64), &'a (PlanetSystem, f64)) {
        let rng = rng.deref_mut();
        let uniform = Uniform::from(0f64..=max);

        let first = {
            let random = uniform.sample(rng);
            Self::selection_wheel(random, values)
        };

        let mut last;
        loop {
            let random = uniform.sample(rng);
            last = Self::selection_wheel(random, values);
            if last != first { break }
        }

        (first, last)
    }

    fn selection_wheel (random: f64, values: &[(PlanetSystem, f64)]) -> &(PlanetSystem, f64) {
        let mut accumulated = 0.;
    
        for slice in values.into_iter() {
            accumulated += slice.1;
            if random <= accumulated { return slice }
        }

        return values.last().unwrap()
    }
}