use std::time::Duration;
use llml::vec::EucVecd2;
use crate::consts::G;

pub trait Body {
    fn get_mass (&self) -> f64; // Jupiter masses
    fn get_pos (&self) -> EucVecd2;
    fn get_vel (&self) -> EucVecd2;

    fn accelerate (&mut self, acc: EucVecd2, dt: Duration);
    fn travel (&mut self, dt: Duration);

    fn accelerate_and_travel (&mut self, acc: EucVecd2, dt: Duration) {
        self.accelerate(acc, dt);
        self.travel(dt)
    }

    /// Returns the acceleration for each element and the direction from ```self```to ```other```
    /// in ```([acc_self, acc_other], dir)```
    fn calc_acc<T: Body> (&self, other: &T) -> (EucVecd2, EucVecd2) {
        let dist = other.get_pos() - self.get_pos();
        let r2 = dist.dot(dist);

        (G * EucVecd2::new([other.get_mass(), self.get_mass()]) / r2, dist.unit())
    }
}