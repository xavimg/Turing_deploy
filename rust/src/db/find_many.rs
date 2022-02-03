use std::{pin::Pin, task::Poll, sync::{Arc, RwLockWriteGuard}, mem::ManuallyDrop, collections::HashSet, ops::DerefMut};
use futures::{Stream, Future};
use mongodb::{error::Error, Cursor};
use crate::{cache::{MongoDoc, DatabaseCache}, CURRENT_LOGGER, Logger};

struct FindManyDbResult<FUT, STR> {
    pub is_stream: bool,
    pub value: FindManyDb<FUT, STR>
}

union FindManyDb<FUT, STR> {
    future: ManuallyDrop<Pin<Box<FUT>>>,
    stream: ManuallyDrop<Pin<Box<STR>>>
}

impl<FUT, STR> Drop for FindManyDbResult<FUT, STR> {
    fn drop(&mut self) {
        if self.is_stream {
            unsafe { ManuallyDrop::<Pin<Box<STR>>>::drop(&mut self.value.stream) }
        } else {
            unsafe { ManuallyDrop::<Pin<Box<FUT>>>::drop(&mut self.value.future) }
        }
    }
}

/// Stream that iterates over the results of both the cache and db searches.\
/// This stream starts iterating with the values of the cache and, one it has available the first
/// batch of results from the database, it starts returning them too
pub struct FindManyStream<T: MongoDoc, CACHE: Stream<Item = Arc<T>>, DB: Future<Output = Result<Cursor<T>, Error>>> {
    cache: Pin<Box<CACHE>>,
    db: FindManyDbResult<DB, Cursor<T>>
}

impl<T: MongoDoc, CACHE: Stream<Item = Arc<T>>, DB: Future<Output = Result<Cursor<T>, Error>>> futures::Stream for FindManyStream<T, CACHE, DB> {
    type Item = (Arc<T>, bool);

    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(x)) = self.cache.as_mut().poll_next(cx) {
            return Poll::Ready(Some((x.clone(), false)))
        }

        let stream;
        if self.db.is_stream {
            stream = unsafe { &mut self.db.value.stream }
        } else {
            if let Poll::Ready(result) = unsafe { (*self.db.value.future).as_mut().poll(cx) } {
                match result {
                    Err(e) => {
                        CURRENT_LOGGER.async_log_error(e);
                        return Poll::Ready(None)
                    },
                    Ok(cursor) => {
                        self.db = FindManyDbResult {
                            is_stream: true, 
                            value: FindManyDb { stream: ManuallyDrop::new(Box::pin(cursor)) }
                        };

                        stream = unsafe { &mut self.db.value.stream }
                    }
                }
            } else {
                return Poll::Pending
            }
        }

        if let Poll::Ready(poll) = stream.as_mut().poll_next(cx) {
            if let Some(res) = poll {
                return match res {
                    Err(e) => { CURRENT_LOGGER.async_log_error(e); Poll::Ready(None) },
                    Ok(val) => {
                        let val = Arc::new(val);
                        return Poll::Ready(Some((val, true)))
                    }
                }
            }

            return Poll::Ready(None)
        }

        Poll::Pending
    }
}

impl<T: MongoDoc> DatabaseCache<T> {
    /// Stream that iterates over the results of both the cache and db searches.\
    /// This stream starts iterating with the values of the cache and, one it has available the first
    /// batch of results from the database, it starts returning them too
    pub(super) fn many_of<'a, CACHE: Stream<Item = Arc<T>>, DB: Future<Output = Result<Cursor<T>, Error>>> (cache: Pin<Box<CACHE>>, db: Pin<Box<DB>>) -> FindManyStream<T, CACHE, DB> {        
        FindManyStream {
            cache,
            db: FindManyDbResult {
                is_stream: false,
                value: FindManyDb { future: ManuallyDrop::new(db) }
            }
        }
    }
}