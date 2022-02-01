use std::{hash::Hash, collections::HashSet, mem::size_of, rc::Rc, sync::{RwLock, Arc}};
use mongodb::{Collection, error::Error, results::InsertOneResult};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use turing_proc::maybee_of;
use crate::filter::DatabaseFilter;

pub struct DatabaseCache<T: Hash + Eq> {
    collection: Rc<Collection<T>>,
    set: RwLock<HashSet<Arc<T>>>
}

impl<T: Hash + Eq + Send + Sync + Serialize + DeserializeOwned + Unpin> DatabaseCache<T> {    
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
            set.insert(Arc::new(value));
        }
    }

    pub async fn insert_one (&mut self, doc: T) -> Result<InsertOneResult, Error> where T: Serialize {
        let result = self.collection.insert_one(&doc, None).await;
        if result.is_ok() {
            self.add_to_cache(doc)
        }

        result
    }

    pub async fn find_one (&mut self, filter: Option<DatabaseFilter<T>>) -> Result<Option<Arc<T>>, Error> {
        let read = self.set.read().unwrap();
        let item = match filter {
            Some(filter) => read.par_iter().find_any(|x| filter.eval(x)),
            None => read.iter().find(|_| true)
        };

        match item {
            Some(x) => Ok(Some(x.clone())),
            None => {
                drop(read);
                match self.collection.find_one(filter.map(|x| x.into()), None).await {
                    Err(e) => Err(e),
                    Ok(x) => {
                        Ok(x)
                    }
                }
            }
        }
    }
}

// MAYBEE
maybee_of!(PartialEq {
    fn eq (&self, rhs: &Self) -> bool;
    fn ne (&self, rhs: &Self) -> bool;
});