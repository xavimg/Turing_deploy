use std::sync::{Mutex};
use std::ops::Deref;

pub trait TakeOut<T> {
    fn take_out (self) -> T;
}

impl<T> TakeOut<T> for Mutex<T> {
    fn take_out (self) -> T {
        let lock = self.lock().unwrap();
        unsafe { std::ptr::read(lock.deref()) }
    }
}

impl<T> TakeOut<T> for Box<T> {
    fn take_out (self) -> T {
        unsafe { std::ptr::read(self.deref()) }
    }
}