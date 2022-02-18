use std::{task::Poll, sync::{Arc}, mem::ManuallyDrop, ops::DerefMut};
use futures::{Stream, Future, FutureExt, StreamExt};
use mongodb::{error::Error, Cursor};
use serde::de::DeserializeOwned;
use crate::{cache::{CollectionCache}, CURRENT_LOGGER, Logger};

struct FindManyDbResult<FUT, STR> {
    pub is_stream: bool,
    pub value: FindManyDb<FUT, STR>
}

union FindManyDb<FUT, STR> {
    future: ManuallyDrop<FUT>,
    stream: ManuallyDrop<STR>
}

impl<FUT, STR> Drop for FindManyDbResult<FUT, STR> {
    fn drop(&mut self) {
        if self.is_stream {
            unsafe { ManuallyDrop::<STR>::drop(&mut self.value.stream) }
        } else {
            unsafe { ManuallyDrop::<FUT>::drop(&mut self.value.future) }
        }
    }
}

/// Stream that iterates over the results of both the cache and db searches.\
/// This stream starts iterating with the values of the cache and, one it has available the first
/// batch of results from the database, it starts returning them too
pub struct FindManyStream<T: DeserializeOwned + Send + Sync + Unpin, CACHE: Unpin + Future<Output = CAHCESTR>, CAHCESTR: Stream<Item = Arc<T>>, DB: Future<Output = Result<Cursor<T>, Error>>> {
    cache: FindManyDbResult<CACHE, CAHCESTR>,
    db: FindManyDbResult<DB, Cursor<T>>
}

impl<T: DeserializeOwned + Send + Sync + Unpin, CACHE: Unpin + Future<Output = CACHESTER>, CACHESTER: Unpin + Stream<Item = Arc<T>>, DB: Unpin + Future<Output = Result<Cursor<T>, Error>>> futures::Stream for FindManyStream<T, CACHE, CACHESTER, DB> {
    type Item = (Arc<T>, bool);

    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let stream;
        if self.db.is_stream {
            stream = unsafe { &mut self.db.value.stream }
        } else if let Poll::Ready(result) = unsafe { self.db.value.future.deref_mut().poll_unpin(cx) } {
            match result {
                Err(e) => {
                    tokio::spawn(CURRENT_LOGGER.log_error(e));
                    return Poll::Ready(None)
                },
                Ok(cursor) => {
                    self.db = FindManyDbResult {
                        is_stream: true, 
                        value: FindManyDb { stream: ManuallyDrop::new(cursor) }
                    };

                    stream = unsafe { &mut self.db.value.stream }
                }
            }
        } else if self.cache.is_stream {
            if let Poll::Ready(cache_result) = unsafe { self.cache.value.stream.deref_mut().poll_next_unpin(cx) } {
                return Poll::Ready(cache_result.map(|x| (x, false)))
            } else {
                return Poll::Pending
            }
        } else if let Poll::Ready(cache_stream) = unsafe { self.cache.value.future.deref_mut().poll_unpin(cx) } {
            self.cache = FindManyDbResult {
                is_stream: true,
                value: FindManyDb { stream: ManuallyDrop::new(cache_stream) }
            };

            return Poll::Pending
        } else {
            return Poll::Pending
        }

        if let Poll::Ready(poll) = stream.poll_next_unpin(cx) {
            if let Some(res) = poll {
                return match res {
                    Err(e) => { tokio::spawn(CURRENT_LOGGER.log_error(e)); Poll::Ready(None) },
                    Ok(val) => {
                        let val = Arc::new(val);
                        return Poll::Ready(Some((val, true)))
                    }
                }
            }

            return Poll::Ready(None)
        } else if self.cache.is_stream {
            if let Poll::Ready(Some(x)) = unsafe { self.cache.value.stream.deref_mut().poll_next_unpin(cx) } {
                return Poll::Ready(Some((x.clone(), false)))
            }
        }

        Poll::Pending
    }
}

impl<T: DeserializeOwned + Send + Sync + Unpin> CollectionCache<T> {
    /// Stream that iterates over the results of both the cache and db searches.\
    /// This stream starts iterating with the values of the cache and, once the first
    /// batch of results from the database is available, it starts returning them too
    
    pub(super) fn many_of<'a, CACHE: Unpin + Future<Output = CACHESTR>, CACHESTR: Unpin + Stream<Item = Arc<T>>, DB: Unpin + Future<Output = Result<Cursor<T>, Error>>> (cache: CACHE, db: DB) -> FindManyStream<T, CACHE, CACHESTR, DB> {        
        FindManyStream {
            cache: FindManyDbResult {
                is_stream: false,
                value: FindManyDb { future: ManuallyDrop::new(cache) }
            },

            db: FindManyDbResult {
                is_stream: false,
                value: FindManyDb { future: ManuallyDrop::new(db) }
            }
        }
    }
}