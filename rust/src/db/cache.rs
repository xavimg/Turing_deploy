use std::{hash::Hash, collections::{HashSet}, sync::{Arc}, ops::{Deref}};
use futures::{StreamExt, Future, future::{select_ok}};
use bson::{oid::ObjectId, doc, Document};
use mongodb::{Collection, error::Error, results::{InsertOneResult, UpdateResult}, options::{UpdateModifications, FindOptions}};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{sync::RwLock, task::JoinError};
use crate::{Streamx, Either, try_spawn};

pub trait MongoDoc {
    fn get_id (&self) -> ObjectId;
}

pub struct CollectionCache<T> {
    collection: Collection<T>,
    set: RwLock<HashSet<Arc<T>>>
}

impl<T: Hash + Eq> CollectionCache<T> {    
    pub fn new (collection: Collection<T>, capacity: usize) -> Self {
        Self {
            collection,
            set: RwLock::new(HashSet::with_capacity(capacity))
        }
    }

    pub async fn insert_one (&self, doc: T) -> Result<(Arc<T>, InsertOneResult), Error> where T: Serialize {
        let insert = self.collection.insert_one(&doc, None).await;
        let doc = Arc::new(doc);
        if insert.is_ok() { self.add_to_cache(doc.clone()); }
        insert.map(move |res| (doc, res))
    }

    /// Inserts element into the cache and retruns a ```Future``` that promises the resolution of the action
    /// on the database. This method is recomended if speed is your ultimate goal, but should be used carefully,
    /// since it means you won't catch database errors until you poll the future
    pub fn insert_one_promise (&self, doc: T) -> (Option<Arc<T>>, impl Future<Output = Result<InsertOneResult, Error>> + '_) where T: Serialize {
        let doc = Arc::new(doc);
        let clone = doc.clone();
        let future = self.collection.insert_one(clone, None);

        if !self.add_to_cache(doc.clone()) {
            return (None, future)
        }

        (Some(doc), future)
    }

    /// Searches for the value with the specified id in the cahche and the database simultaneously, returning the result of
    /// the first search to complete and killing the other
    pub async fn find_any_one (&'static self) -> Result<Option<Arc<T>>, Either<JoinError, Error>> where T: Unpin + Send + Sync + DeserializeOwned {
        self.find_one(doc! {}, |_| true).await
    }
    
    pub async fn find_one_by_id (&'static self, id: ObjectId) -> Result<Option<Arc<T>>, Either<JoinError, Error>> where T: MongoDoc + Unpin + Send + Sync + DeserializeOwned {
        self.find_one(doc! { "_id": id }, move |x| x.get_id() == id).await
    }

    /// Searches for the value with the specified parameters in the cahche and the database simultaneously, returning the result of
    /// the first search to complete and killing the other
    pub async fn find_one<F: 'static + Send + Sync + Fn(&T) -> bool> (&'static self, db: Document, cache: F) -> Result<Option<Arc<T>>, Either<JoinError, Error>> where T: Unpin + Send + Sync + DeserializeOwned {
        let cache_fn = Arc::new(cache);
        let cache = try_spawn(async move {
            let read = self.set.read().await;
            let handles = read.iter().cloned().map(|entry| -> crate::TrySpawn<Arc<T>, ()> {
                let my_cache = cache_fn.clone();
                try_spawn(async move {
                    if my_cache(&entry) { return Ok(entry) }
                    Err(())
                })
            });

            match select_ok(handles).await {
                Err(e) => Err(e.map_left(|e| Either::Left(e))),
                Ok((res, futs)) => {
                    futs.into_iter().for_each(|fut| fut.handle.abort());
                    Ok(res)
                }
            }
        });

        let db = try_spawn(async { 
            match self.collection.find_one(db, None).await {
                Err(e) => Err(Either::Left(Either::Right(e))),
                Ok(x) => match x {
                    None => Err(Either::Right(())),
                    Some(x) => Ok(Arc::new(x))
                }
            }
        });

        match select_ok([cache, db]).await {
            Err(err) => match err {
                Either::Left(join) => Err(Either::Left(join)),
                Either::Right(either) => match either {
                    Either::Left(e) => Err(e),
                    Either::Right(_) => Ok(None)
                }
            },

            Ok((result, other)) => {
                other[0].handle.abort();
                Ok(Some(result))
            }
        }
    }

    /// Searches for the values with the specified parameters in the cache and the database simultaneously, returning the result of
    /// the first individual search to complete
    pub async fn find_many<F: 'static + Fn(&T) -> bool> (&'static self, db: Document, cache: F, limit: Option<usize>) -> HashSet<Arc<T>> where T: Unpin + Send + Sync + DeserializeOwned {
        let db_opts = match limit {
            None => FindOptions::default(),
            Some(len) => {
                let mut opts = FindOptions::default();
                opts.limit = Some(len as i64);
                opts
            }
        };
        
        let lock = self.set.read().await;
        let cache = futures::stream::iter(lock.deref().iter()).async_filter(|x| cache(x.deref())).cloned();
        let db = self.collection.find(db, db_opts);

        let mut stream = Self::many_of(Box::pin(cache), Box::pin(db));
        let mut results;
        let mut dbs;

        match limit {
            Some(len) => {
                results = HashSet::with_capacity(len);
                dbs = Vec::with_capacity(len);

                while results.len() < len {
                    if let Some((x, from_db)) = stream.next().await {
                        if from_db { dbs.push(x.clone()); }
                        results.insert(x);
                        continue
                    } 

                    break
                }
            },

            None => {
                results = HashSet::new();
                dbs = Vec::new();

                while let Some((x, from_db)) = stream.next().await {
                    if from_db { dbs.push(x.clone()); }
                    results.insert(x);
                }
            }
        }

        drop(lock);
        self.add_all_to_cache(dbs);
        results
    }

    pub async fn find_any (&'static self, limit: Option<usize>) -> HashSet<Arc<T>> where T: Unpin + Send + Sync + DeserializeOwned {
        self.find_many(doc! {}, |_| true, limit).await
    }

    pub async fn update_one (&self, filter: Document, update: impl Into<UpdateModifications>) -> Result<Option<Arc<T>>, Error> where T: DeserializeOwned {
        self.collection.find_one_and_update(filter, update, None).await.map(|x| {
            x.map(|x| {
                let x = Arc::new(x);
                let mut lock = self.set.write().unwrap();
                lock.replace(x.clone());
                x
            })
        })
    }

    /// Updates element in the cache and retruns a ```Future``` that promises the resolution of the action
    /// on the database. This method is recomended if speed is your ultimate goal, but should be used carefully,
    /// since it means you won't catch database errors until you poll the future
    pub fn update_one_promise<F1: Send + Sync + Fn(&T) -> bool, F2: FnOnce(T) -> T> (&self, filter_db: Document, filter_cache: F1, update_db: impl Into<UpdateModifications>, update_cache: F2) -> (Option<Arc<T>>, impl Future<Output = Result<UpdateResult, Error>> + '_) where T: Send + Sync + Clone {
        let future = self.collection.update_one(filter_db, update_db.into(), None);
        let lock = self.set.read().unwrap();

        let result = lock.par_iter()
            .find_any(|elem| filter_cache(elem.deref()))
            .map(|value| update_cache(value.deref().clone()));

        match result {
            Some(value) => {
                let value = Arc::new(value);
                drop(lock);

                let mut lock = self.set.write().unwrap();
                lock.replace(value.clone());
                (Some(value), future)
            },

            None => (None, future)
        }
    }
}

impl<T> CollectionCache<T> {
    pub(super) fn add_to_cache (&self, value: Arc<T>) -> bool where T: Hash + Eq {
        let mut set = self.set.write().unwrap();
        if set.len() < set.capacity() {
            return set.insert(value)
        }

        false
    }

    pub(super) fn add_all_to_cache<I: IntoIterator<Item = Arc<T>>> (&self, value: I) -> bool where T: Hash + Eq {
        let value = value.into_iter();
        let (_, hint) = value.size_hint();

        return match hint {
            Some(len) => {
                let mut set = self.set.write().unwrap();
                let max_len = set.capacity() - set.len();

                let mut value = value.take(len.min(max_len));
                let mut all = true;
                while let Some(val) = value.next() {
                    all &= set.insert(val)
                }

                all
            },

            None => value.map(|x| self.add_to_cache(x)).all(|x| x)
        }
    }
}