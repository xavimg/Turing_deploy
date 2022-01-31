use std::{ops::Deref, hash::Hash, collections::HashSet, mem::size_of, rc::Rc, sync::RwLock, time::Duration};
use mongodb::{Collection, error::Error, results::InsertOneResult};
use serde::{Serialize, Deserialize};

use crate::filter::DatabaseFilter;

pub struct DatabaseCache<T: Hash + Eq> {
    collection: Rc<Collection<T>>,
    set: RwLock<HashSet<T>>
}

impl<T: Hash + Eq> DatabaseCache<T> {    
    pub fn new (collection: Rc<Collection<T>>, capacity: usize) -> Self {
        Self {
            collection,
            set: RwLock::new(HashSet::with_capacity(capacity))
        }
    }

    pub fn from_byte_capacity (collection: Rc<Collection<T>>, capacity: usize) -> Self {
        Self::new(collection, capacity / size_of::<T>())
    }

    fn add_to_cache (&mut self, value: T) {
        let mut set = self.set.write().unwrap();
        if set.len() < set.capacity() {
            set.insert(value);
        }
    }

    pub async fn insert_one (&mut self, doc: T) -> Result<InsertOneResult, Error> where T: Serialize {
        let result = self.collection.insert_one(&doc, None).await;
        if result.is_ok() {
            self.add_to_cache(doc)
        }

        result
    }

    pub async fn find_one<'a> (&mut self, filter: Option<DatabaseFilter<T>>) -> Result<(), Error> where T: Deserialize<'a> {
        match &filter {
            Some(x) => {
                let read = self.set.read().unwrap();
            }
        }

        todo!()
    }
}