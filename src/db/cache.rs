use std::{hash::Hash, collections::{HashSet}, sync::{Arc}, ops::{Deref}, future::ready, pin::Pin};
use futures::{StreamExt, Future, future::{select_ok}, FutureExt, Stream};
use bson::{oid::ObjectId, doc, Document};
use mongodb::{Collection, error::Error, results::{InsertOneResult, UpdateResult}, options::{UpdateModifications, FindOptions}};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{sync::{RwLock}, task::JoinError, join};
use xstd::stream::UniqueBy;
use crate::{Streamx, Either, try_spawn, CURRENT_LOGGER, Logger};

pub trait MongoDoc {
    fn get_id (&self) -> ObjectId;
}
pub struct CollectionCache<T> {
    collection: Collection<T>,
    set: Arc<RwLock<HashSet<Arc<T>>>>
}

impl<T: Hash + Eq> CollectionCache<T> {
    #[inline]
    pub fn new (collection: Collection<T>, capacity: usize) -> Self {
        Self {
            collection,
            set: Arc::new(RwLock::new(HashSet::with_capacity(capacity)))
        }
    }

    pub async fn insert_one (&self, doc: T) -> Result<Arc<T>, Error> where T: Serialize {
        let doc = Arc::new(doc);
        let (_, db) = join!(self.add_to_cache(doc.clone()), self.collection.insert_one(doc.clone(), None));

        match db {
            Err(e) => Err(e),
            Ok(_) => Ok(doc)
        }
    }

    /// Inserts element into the cache and retruns a ```Future``` that promises the resolution of the action
    /// on the database. This method runs unther the reasonable assumption that the cache will be faster than the database, and is recomended if speed is your ultimate goal, but should be used carefully,
    /// since it means you won't catch database errors until you poll the future
    pub async fn insert_one_promise (&self, doc: T) -> (Option<Arc<T>>, impl Future<Output = Result<InsertOneResult, Error>> + '_) where T: Serialize {
        let doc = Arc::new(doc);
        let future = self.collection.insert_one(doc.clone(), None);

        if !self.add_to_cache(doc.clone()).await {
            return (None, future)
        }

        (Some(doc), future)
    }

    /// Searches for the value with the specified id in the cahche and the database simultaneously, returning the result of
    /// the first search to complete and killing the other
    #[inline]
    pub async fn find_any_one (&'static self) -> Result<Option<Arc<T>>, Either<JoinError, Error>> where T: Unpin + Send + Sync + DeserializeOwned {
        self.find_one(doc! {}, |_| true).await
    }

    #[inline]
    pub async fn find_one_by_id (&'static self, id: ObjectId) -> Result<Option<Arc<T>>, Either<JoinError, Error>> where T: MongoDoc + Unpin + Send + Sync + DeserializeOwned {
        self.find_one(doc! { "_id": id }, move |x| x.get_id() == id).await
    }

    #[inline]
    pub async fn find_one_by_value (&'static self, value: &T) -> Result<Option<Arc<T>>, Either<JoinError, Error>> where T: MongoDoc + Unpin + Send + Sync + DeserializeOwned {
        self.find_one_by_id(value.get_id()).await
    }

    /// Searches for the value with the specified parameters in the cahche and the database simultaneously, returning the result of
    /// the first search to complete and killing the other
    pub async fn find_one<F: 'static + Send + Sync + Fn(&T) -> bool> (&'static self, db: Document, cache: F) -> Result<Option<Arc<T>>, Either<JoinError, Error>> where T: Unpin + Send + Sync + DeserializeOwned {
        let cache_fn = Arc::new(cache);
        let cache = try_spawn(async move {
            let read = self.set.read().await;
            let mut handles = read.iter().cloned().map(|entry| -> crate::TrySpawn<Arc<T>, ()> {
                let my_cache = cache_fn.clone();
                try_spawn(async move {
                    if my_cache(&entry) { return Ok(entry) }
                    Err(())
                })
            }).peekable();

            if let None = handles.peek() {
                return Err(Either::Right(()))
            }

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
                if let Some(other) = other.get(0) {
                    other.handle.abort();
                }

                Ok(Some(result))
            }
        }
    }

    /// Searches for the values with the specified parameters in the cache and the database simultaneously, returning the result of
    /// the first individual search to complete
    pub fn find_many<F: 'static + Unpin + Fn(&T) -> bool> (&'static self, db: Document, cache: F, limit: Option<usize>) -> impl Stream<Item = Arc<T>> where T: Unpin + Send + Sync + Hash + Eq + DeserializeOwned {
        let db_opts = match limit {
            None => FindOptions::default(),
            Some(len) => {
                let mut opts = FindOptions::default();
                opts.limit = Some(len as i64);
                opts
            }
        };
        
        let cache = self.set.read().map(|lock| {
            let ptr : *const HashSet<Arc<T>> = lock.deref(); // ```self``` has static lifetime, so this isn't undefined behavior
            futures::stream::iter(unsafe { &*ptr }).async_filter(move |x| cache(x.deref())).cloned()
        });

        let db = async move {
            let stream : Pin<Box<dyn Stream<Item = Arc<T>>>> = match self.collection.find(db, db_opts).await {
                Ok(cursor) => Box::pin(cursor.filter(|x| ready(x.is_ok())).map(move |x| {
                    let arc = Arc::new(x.unwrap());
                    let arc_clone = arc.clone();

                    tokio::spawn(async move {
                        let mut lock = self.set.write().await;
                        if lock.len() < lock.capacity() { lock.insert(arc); }
                    });

                    arc_clone
                })),

                Err(e) => {
                    tokio::spawn(CURRENT_LOGGER.log_error(format!("{e}")));
                    Box::pin(futures::stream::empty())
                }
            }; 
            stream
        };

        Box::pin(db).flatten_stream()
            .merge(Box::pin(cache).flatten_stream())
            .unique_by(|x| x.clone())
    }

    #[inline]
    pub fn find_any (&'static self, limit: Option<usize>) -> impl Stream<Item = Arc<T>> where T: Unpin + Send + Sync + DeserializeOwned {
        self.find_many(doc! {}, |_| true, limit)
    }

    pub async fn update_one<F: 'static + Send + Sync + Fn(&T) -> bool> (&'static self, filter: Document, cache: F, update: impl Into<UpdateModifications>) -> Result<Option<Arc<T>>, Either<JoinError, Error>> where T: Unpin + Send + Sync + MongoDoc + DeserializeOwned {
        match self.find_one(filter, cache).await {
            Ok(Some(x)) => {
                let id = doc! { "_id": x.get_id() };
                match self.collection.update_one(id.clone(), update, None).await {
                    Ok(_) => match self.collection.find_one(id, None).await {
                        Ok(Some(x)) => {
                            let arc = Arc::new(x);
                            let mut lock = self.set.write().await;
                            lock.replace(arc.clone());
                            Ok(Some(arc))
                        },

                        Ok(None) => Ok(None),
                        Err(e) => Err(Either::Right(e))
                    },

                    Err(e) =>  Err(Either::Right(e))
                }
            },

            Ok(None) => Ok(None),
            Err(e) => Err(e)
        }

        /*match self.collection.find_one_and_update(filter, update, None).await {
            Err(e) => {
                Err(e)
            },
            Ok(x) => Ok(match x {
                None => None,
                Some(x) => {
                    let x = Arc::new(x);
                    let mut lock = self.set.write().await;
                    lock.replace(x.clone());
                    Some(x)
                }
            })
        }*/
    }

    /// Updates element in the cache and retruns a ```Future``` that promises the resolution of the action
    /// on the database. This method is recomended if speed is your ultimate goal, but should be used carefully,
    /// since it means you won't catch database errors until you poll the future
    pub async fn update_one_promise<F1: 'static + Send + Sync + Fn(&T) -> bool, F2: FnOnce(T) -> T> (&self, filter_db: Document, filter_cache: F1, update_db: impl Into<UpdateModifications>, update_cache: F2) -> (Result<Option<Arc<T>>, JoinError>, impl Future<Output = Result<UpdateResult, Error>> + '_) where T: 'static + Send + Sync + Clone {
        let future = self.collection.update_one(filter_db, update_db.into(), None);
        let lock = self.set.read().await;

        let filter_cache = Arc::new(filter_cache);
        let handles = lock.iter().cloned().map(|entry| -> crate::TrySpawn<Arc<T>, ()> {
            let my_cache = filter_cache.clone();
            try_spawn(async move {
                if my_cache(&entry) { return Ok(entry) }
                Err(())
            })
        });

        let res = select_ok(handles).await;
        drop(lock);

        let res = match res {
            Err(e) => match e {
                Either::Left(e) => Err(e),
                Either::Right(_) => Ok(None)
            },

            Ok((res, futs)) => {
                futs.into_iter().for_each(|fut| fut.handle.abort());
                let res = Arc::new(update_cache(res.deref().clone()));
                let mut lock = self.set.write().await;
                lock.replace(res.clone());
                Ok(Some(res))
            }
        };

        (res, future)
    }
}

impl<T> CollectionCache<T> {
    pub(super) async fn add_to_cache (&self, value: Arc<T>) -> bool where T: Hash + Eq {
        let mut set = self.set.write().await;
        if set.len() < set.capacity() {
            return set.insert(value)
        }

        false
    }

    pub(super) async fn add_all_to_cache<I: IntoIterator<Item = Arc<T>>> (&self, value: I) -> bool where T: Hash + Eq {
        let value = value.into_iter();
        let (_, hint) = value.size_hint();

        return match hint {
            Some(len) => {
                let mut set = self.set.write().await;
                let max_len = set.capacity() - set.len();

                let mut value = value.take(len.min(max_len));
                let mut all = true;
                while let Some(val) = value.next() {
                    all &= set.insert(val)
                }

                all
            },

            None => {
                for item in value {
                    if !self.add_to_cache(item).await { return false }
                }

                true
            }
        }
    }
}