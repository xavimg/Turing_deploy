use rayon::iter::{ParallelBridge, ParallelIterator};


struct LinearSpace {
    at: f64,
    to: f64,
    delta: f64
}

impl LinearSpace {
    pub fn new (from: f64, to: f64, len: usize) -> Self {
        LinearSpace {
            at: from,
            to,
            delta: (to - from) / (len as f64)
        }
    }
}

impl Iterator for LinearSpace {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at >= self.to {
            return None
        }

        let ret = Some(self.at);
        self.at += self.delta;
        ret
    }
}

pub fn integral<F: Fn(f64) -> f64> (from: f64, to: f64, len: usize, f: F) -> f64 where F: Send + Sync {
    let dist = to - from;
    let cast = len as f64;
    let n2 = 2. * cast;

    let delta = dist / cast;
    let beta = dist / n2;

    let lnsp = LinearSpace {
        at: from + beta,
        to: to - beta,
        delta
    };

    let sum : f64 = lnsp.par_bridge()
        .map(|x| f(x))
        .sum();

    sum * delta
} 

pub fn loop_clamp<T: PartialOrd, F: FnMut() -> T> (min: T, max: T, mut f: F) -> T {
    loop {
        let value = f();
        if value < min || value > max {
            continue
        }

        return value
    }
}