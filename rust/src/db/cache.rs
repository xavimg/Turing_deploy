use std::{hash::Hash, collections::HashSet, sync::{RwLock, Arc}};
use actix_web::Either;
use futures::{join, select, FutureExt, pin_mut, try_join, future::{TryJoin, try_maybe_done}};
use bson::{oid::ObjectId, doc, Document};
use mongodb::{Collection, error::Error, results::{InsertOneResult}, options::UpdateModifications};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use serde::{Serialize, de::DeserializeOwned};
use crate::{CURRENT_LOGGER, find::Either2, EitherOrAll};
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

    fn add_to_cache (&self, value: Arc<T>) {
        let mut set = self.set.write().unwrap();
        if set.len() < set.capacity() {
            set.insert(value);
        }
    }

    pub async fn insert_one (&self, doc: T) -> Result<InsertOneResult, Error> {
        let insert = self.collection.insert_one(&doc, None).await;
        if insert.is_ok() { self.add_to_cache(Arc::new(doc)); }
        insert
    }

    /// Searches for the value with the specified id in the cahche and the database simultaneously, returning the result of
    /// the first search to complete and killing the other
    pub async fn find_one_by_id (&self, id: ObjectId) -> Result<Option<Arc<T>>, Error> {
        let cache = async {
            let read = self.set.read().unwrap();
            match read.par_iter().find_any(|x| x.get_id() == id) {
                None => Err(()),
                Some(x) => Ok(x.clone())
            }
        }.fuse();

        let db = async {
            match self.collection.find_one(doc! { "_id": id }, None).await {
                Err(e) => Err(EitherOrAll::Left(e)),
                Ok(x) => match x {
                    None => Err(EitherOrAll::Right(())),
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
                EitherOrAll::Left(e) => Err(e),
                _ => Ok(None)
            }
        }
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