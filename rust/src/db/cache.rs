use std::{hash::Hash, collections::HashSet, sync::{RwLock, Arc}, thread};
use futures::{join, select, FutureExt, pin_mut};
use bson::{Document, oid::ObjectId, doc};
use lazy_static::__Deref;
use mongodb::{Collection, error::Error, results::{InsertOneResult}, options::UpdateModifications};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use serde::{Serialize, de::DeserializeOwned};
use crate::{filter::{DatabaseFilter}};
use crate::CURRENT_LOGGER;
use crate::Logger;

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

    async fn add_to_cache (&self, value: Arc<T>) {
        let mut set = self.set.write().unwrap();
        if set.len() < set.capacity() {
            set.insert(value);
        }
    }

    pub async fn insert_one (&'static self, doc: T) -> Result<InsertOneResult, Error> {
        let insert = self.collection.insert_one(&doc, None).await;
        if insert.is_ok() { thread::spawn(|| self.add_to_cache(Arc::new(doc))); }
        insert
    }

    /// Searches for the value with the specified id in the cahche and the database simultaneously, returning the result of
    /// the first search to complete and killing the other
    pub async fn find_one_by_id (&'static self, id: ObjectId) -> Result<Option<Arc<T>>, Error> {
        let cache = async {
            let read = self.set.read().unwrap();
            read.par_iter().find_any(|x| x.get_id() == id).map(|x| x.clone())
        }.fuse();

        let db = self.collection.find_one(doc! { "_id": id }, None).fuse();
        pin_mut!(cache, db);

        select!(
            result = cache => {
                CURRENT_LOGGER.async_log_info(format!("Polled {id:?} from cache"));
                Ok(result)
            },

            result = db => {
                CURRENT_LOGGER.async_log_info(format!("Polled {id:?} from database"));
                result.map(|ok| ok.map(|val| {
                    let val = Arc::new(val);
                    let clone = val.clone();

                    thread::spawn(|| self.add_to_cache(clone));
                    val
                }))
            }
        )
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