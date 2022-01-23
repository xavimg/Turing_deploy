use std::intrinsics::transmute;
use llml::vec::{EucVecd4};

const DELTA : f64 = f32::EPSILON as f64;
const DELTA4 : f64 = 4. * DELTA;
const DELTA_VEC : EucVecd4 = unsafe { transmute([0., DELTA, 2. * DELTA, 3. * DELTA]) };

// FUNCTION
pub fn integrate<F: Fn(f64) -> f64> (from: f64, to: f64, f: F) -> f64 {
    let mut x = from;
    let mut y = 0.;

    while x <= to {
        let (x1, x2, x3, x4) = (EucVecd4::from_scal(x) + DELTA_VEC).unzip();
        let vec = EucVecd4::new([f(x1), f(x2), f(x3), f(x4)]);
        y += vec.sum();
        x += DELTA4;
    }

    y * DELTA4
}

#[test]
fn integral () {
    assert_eq!(integrate(2., 3., |x| x.ln()), 0.909542504884)
}