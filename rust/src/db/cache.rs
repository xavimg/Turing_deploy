use std::{hash::Hash, collections::HashSet, mem::size_of, sync::{RwLock, Arc}, thread};
use bson::{Document, oid::ObjectId, doc};
use lazy_static::__Deref;
use mongodb::{Collection, error::Error, results::{InsertOneResult}, options::UpdateModifications};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use serde::{Serialize, de::DeserializeOwned};
use crate::{filter::{DatabaseFilter}};

pub trait MongoDoc: Hash + Eq + Send + Sync + Serialize + DeserializeOwned + Unpin {
    fn get_id (&self) -> ObjectId;
}

pub struct DatabaseCache<T> {
    collection: Collection<T>,
    set: RwLock<HashSet<Arc<T>>>
}

impl<T: MongoDoc> DatabaseCache<T> {    
    pub fn new (collection: Collection<T>, capacity: usize) -> Self {
        Self {
            collection,
            set: RwLock::new(HashSet::with_capacity(capacity))
        }
    }

    fn add_to_cache (&self, value: Arc<T>) {
        let mut set = self.set.write().unwrap();
        if set.len() < set.capacity() {
            set.insert(value);
        }
    }

    pub async fn insert_one (&'static self, doc: T) -> Result<InsertOneResult, Error> {
        let result = self.collection.insert_one(&doc, None).await;
        if result.is_ok() { thread::spawn(|| self.add_to_cache(Arc::new(doc))); }
        result
    }

    pub async fn find_one_by_id (&'static self, id: ObjectId) -> Result<Option<Arc<T>>, Error> {
        let read = self.set.read().unwrap();
        let item = read.par_iter().find_any(|x| x.get_id() == id);

        match item {
            None => {
                drop(read);
                self.collection.find_one(doc! { "_id": id }, None).await.map(|x| {
                    x.map(|x| {
                        let x = Arc::new(x);
                        let y = x.clone();
                        thread::spawn(move || self.add_to_cache(y));
                        x
                    })
                })
            },

            Some(x) => Ok(Some(x.clone())),
        }
    }

    pub async fn find_one<F: Send + Sync + Fn(&T) -> bool> (&self, filter: Option<DatabaseFilter<T,F>>) -> Result<Option<Arc<T>>, Error> {
        let read = self.set.read().unwrap();
        let item = match &filter {
            Some(filter) => read.par_iter().find_any(|x| (filter.cache)(x.deref())),
            None => read.iter().find(|_| true)
        };

        match item {
            Some(x) => Ok(Some(x.clone())),
            None => {
                drop(read);
                match self.collection.find_one(filter.map(|x| x.db), None).await {
                    Err(e) => Err(e),
                    Ok(x) => {
                        let res = match x {
                            Some(x) => {
                                let x = Arc::new(x);
                                self.add_to_cache(x.clone());
                                Some(x)
                            },
                            None => None
                        };

                        Ok(res)
                    }
                }
            }
        }
    }

    pub async fn update_one (&'static self, filter: Document, update: impl Into<UpdateModifications>) -> Result<Option<ObjectId>, Error> {
        self.collection.find_one_and_update(filter, update, None).await.map(|x| {
            x.map(|x| {
                let id = x.get_id();
                let x = Arc::new(x);
                thread::spawn(|| {
                    let mut lock = self.set.write().unwrap();
                    lock.replace(x);
                });

                id
            })
        })
    }
}