use std::rc::Rc;
use serde::{Serialize};

pub struct RcWrapper<T>(Rc<T>);

impl<T: Serialize> Serialize for RcWrapper<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.0.as_ref().serialize(serializer)
    }
}