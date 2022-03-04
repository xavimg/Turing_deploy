use std::{sync::{Arc, RwLock}};
use crate::{Threadly};

#[repr(transparent)]
pub struct SceneReader<T>(Threadly<T>);

impl<T> From<Threadly<T>> for SceneReader<T> {
    fn from(x: Threadly<T>) -> Self {
        SceneReader(x)
    }
}

impl<T> From<T> for SceneReader<T> {
    fn from(x: T) -> Self {
        Arc::new(RwLock::new(x)).into()
    }
}

#[repr(transparent)]
pub struct SceneWriter<T>(Threadly<T>);

impl<T> From<Threadly<T>> for SceneWriter<T> {
    fn from(x: Threadly<T>) -> Self {
        SceneWriter(x)
    }
}

impl<T> From<T> for SceneWriter<T> {
    fn from(x: T) -> Self {
        Arc::new(RwLock::new(x)).into()
    }
}

#[macro_export]
macro_rules! scene_with {
    ($($value:expr => $ty:ty),+) => {
        {
            struct Scene($($ty,)*);
            Scene($($value.into(),)*)

            impl Scene {
                pub fn start (self) {
                    std::thread::spawn(move || {
                        loop {
                            //let input = ($($)*);
                        }
                    })
                }
            }
        }
    };
}