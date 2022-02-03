use std::{hash::Hash, collections::HashSet, sync::{RwLock, Arc, RwLockReadGuard, RwLockWriteGuard}, pin::Pin, fmt::Debug, ops::DerefMut};
use actix_web::Either;
use std::ops::Deref;
use futures::{FutureExt, pin_mut, Stream, Future};
use bson::{oid::ObjectId, doc, Document};
use mongodb::{Collection, error::Error, results::{InsertOneResult}, options::{UpdateModifications}};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use serde::{Serialize, de::DeserializeOwned};
use futures::stream::{StreamExt};

pub trait MongoDoc: Debug + Hash + Eq + Send + Sync + Serialize + DeserializeOwned + Unpin {
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

    pub(super) fn add_to_cache (&self, value: Arc<T>) -> bool {
        let mut set = self.set.write().unwrap();
        if set.len() < set.capacity() {
            return set.insert(value)
        }

        false
    }

    pub(super) fn add_all_to_cache<I: IntoIterator<Item = Arc<T>>> (&self, value: I) -> bool {
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

    pub async fn insert_one (&self, doc: T) -> Result<InsertOneResult, Error> {
        let insert = self.collection.insert_one(&doc, None).await;
        if insert.is_ok() { self.add_to_cache(Arc::new(doc)); }
        insert
    }

    /// Searches for the value with the specified id in the cahche and the database simultaneously, returning the result of
    /// the first search to complete and killing the other
    pub async fn find_any_one (&self) -> Result<Option<Arc<T>>, Error> {
        self.find_one(doc! {}, |_| true).await
    }
    
    pub async fn find_one_by_id (&self, id: ObjectId) -> Result<Option<Arc<T>>, Error> {
        self.find_one(doc! { "_id": id }, |x| x.get_id() == id).await
    }

    /// Searches for the value with the specified parameters in the cahche and the database simultaneously, returning the result of
    /// the first search to complete and killing the other
    pub async fn find_one<F: Send + Sync + Fn(&T) -> bool> (&self, db: Document, cache: F) -> Result<Option<Arc<T>>, Error> {
        let cache = async {
            let read = self.set.read().unwrap();
            match read.par_iter().find_any(|x| (cache)(x.deref())) {
                None => Err(()),
                Some(x) => Ok(x.clone())
            }
        }.fuse();

        let db = async { 
            match self.collection.find_one(db, None).await {
                Err(e) => Err(Either::Left(e)),
                Ok(x) => match x {
                    None => Err(Either::Right(())),
                    Some(x) => Ok(Arc::new(x))
                }
            }
        }.fuse();

        match Self::any_of(Box::pin(cache), Box::pin(db)).await {
            Ok(either) => match either {
                Either::Left(x) => Ok(Some(x)),
                Either::Right(x) => {
                    self.add_to_cache(x.clone());
                    Ok(Some(x))
                }
            },

            Err((_, e)) => match e {
                Either::Left(e) => Err(e),
                _ => Ok(None)
            }
        }
    }

    /// Searches for the values with the specified parameters in the cache and the database simultaneously, returning the result of
    /// the first individual search to complete
    pub async fn find_many<F: 'static + Fn(&T) -> FUT, FUT: 'static + Future<Output = bool>> (&'static self, db: Document, cache: F, limit: Option<usize>) -> Vec<Arc<T>> {
        let lock = self.set.read().unwrap();
        let cache = futures::stream::iter(lock.deref().iter().cloned()).filter(move |x| cache(x.deref()));
        let db = self.collection.find(db, None);

        let mut stream = Self::many_of(Box::pin(cache), Box::pin(db));
        let mut results = Vec::<Arc<T>>::with_capacity(match limit {
            None => 10,
            Some(len) => len
        });

        let mut dbs = Vec::<Arc<T>>::with_capacity(match limit {
            None => 10,
            Some(len) => len
        });

        while let Some((x, from_db)) = stream.next().await { 
            if from_db { dbs.push(x.clone()); }
            results.push(x)
        };

        drop(lock);
        self.add_all_to_cache(dbs);

        results
    }

    pub async fn update_one (&self, filter: Document, update: impl Into<UpdateModifications>) -> Result<Option<ObjectId>, Error> {
        self.collection.find_one_and_update(filter, update, None).await.map(|x| {
            x.map(|x| {
                let id = x.get_id();
                let x = Arc::new(x);

                let mut lock = self.set.write().unwrap();
                lock.replace(x);

                id
            })
        })
    }
}