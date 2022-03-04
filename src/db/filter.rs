use std::marker::PhantomData;
use bson::{Document};
use mongodb::options::UpdateModifications;

// FILTER
pub struct DatabaseFilter<T, F> {
    pub cache: F,
    pub db: Document,
    phantom: PhantomData<T>
}

pub struct DatabaseUpdate<T, F> {
    pub cache: F,
    pub db: UpdateModifications,
    phantom: PhantomData<T>
}

impl<T, F: Fn(&T) -> bool> DatabaseFilter<T,F> {
    pub fn new (cache: F, db: Document) -> Self {
        Self {
            cache,
            db,
            phantom: PhantomData
        }
    }
}

impl<T, F: FnMut(&mut T)> DatabaseUpdate<T,F> {
    pub fn new (cache: F, db: UpdateModifications) -> Self {
        Self {
            cache,
            db,
            phantom: PhantomData
        }
    }
}