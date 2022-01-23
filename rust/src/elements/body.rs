use llml::vec::EucVecd3;

pub trait Body {
    fn get_mass (&self) -> f64;
    fn get_pos (&self) -> EucVecd3;
    fn get_vel (&self) -> EucVecd3;

    fn 
}