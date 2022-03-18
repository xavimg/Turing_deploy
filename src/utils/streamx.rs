use std::{pin::Pin, task::Poll, collections::{HashSet}, hash::Hash, sync::Arc};
use futures::{Stream, Future, FutureExt, StreamExt};
use tokio::task::{JoinHandle, JoinError};
use crate::Either;

pub trait Streamx: Stream {
    /// Async version of ```Iterator::cloned```
    #[inline]
    fn cloned<'a, T: Clone> (self) -> StreamCloned<'a, T, Self> where Self: Stream<Item = &'a T> + Sized {
        StreamCloned(self)
    }

    /// Merges results from two streams into one unorderedly
    #[inline]
    fn merge<S: Stream<Item = Self::Item>> (self, other: S) -> Merge<Self, S> where Self: Sized {
        Merge { first: self, last: other }
    }

    /// Asynchronus filter. Unlike it's ```futures``` counterpart, this stream will report itself as ```Poll::Pending```
    /// until it finds matching value
    #[inline]
    fn async_filter<F: Fn(&Self::Item) -> bool> (self, predicate: F) -> AsyncFilter<Self, F> where Self: Sized {
        AsyncFilter {
            stream: self,
            filter: predicate
        }
    }

    /// Returns only the unique values of the stream
    #[inline]
    fn unique (self) -> Unique<Self> where Self::Item: Hash + Eq, Self: Sized {
        let (min, max) = self.size_hint();
        let len = match max {
            Some(max) => max,
            None => min
        };

        Unique {
            set: HashSet::with_capacity(len),
            stream: self
        }
    }

    /// Returns only the unique values of the stream
    #[inline]
    fn unique_arc<T: Hash + Eq> (self) -> UniqueArc<Self> where Self: Stream<Item = Arc<T>> + Sized {
        let (min, max) = self.size_hint();
        let len = match max {
            Some(max) => max,
            None => min
        };

        UniqueArc {
            set: HashSet::with_capacity(len),
            stream: self
        }
    }
}

impl<T: Stream> Streamx for T {}

pub struct AsyncFilter<S: Stream, F: Fn(&S::Item) -> bool> {
    stream: S,
    filter: F
}

impl<S: Unpin + Stream, F: Fn(&S::Item) -> bool> Stream for AsyncFilter<S, F> where Self: Unpin {
    type Item = S::Item;

    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        match self.stream.poll_next_unpin(cx) {
            Poll::Ready(Some(value)) => {
                if (self.filter)(&value) { return Poll::Ready(Some(value)) }
                return Poll::Pending
            },
            res => res
        }
    }
}

pub struct StreamCloned<'a, T: 'a + Clone, S: Stream<Item = &'a T>>(S);

impl<'a, T: 'a + Clone, S: Unpin + Stream<Item = &'a T>> Stream for StreamCloned<'a,T,S> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(result) = self.0.poll_next_unpin(cx) {
            return Poll::Ready(result.map(|x| x.clone()))
        }

        Poll::Pending
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

pub fn try_spawn<O: 'static + Send, E: 'static + Send, T: Future<Output = Result<O,E>> + Send + 'static> (fut: T) -> TrySpawn<O, E> {
    TrySpawn {
        handle: tokio::spawn(fut)
    }
}

pub struct TrySpawn<T, E> {
    pub handle: JoinHandle<Result<T,E>>
}

impl<T, E> Future for TrySpawn<T,E> {
    type Output = Result<T, Either<JoinError,E>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        self.handle.poll_unpin(cx).map(|res| match res {
            Err(e) => Err(Either::Left(e)),
            Ok(res) => res.map_err(|e| Either::Right(e))
        })
    }
}

/// Merge 2 streams unorderedly
pub struct Merge<A,B> {
    first: A,
    last: B
}

impl<T, A: Unpin + Stream<Item = T>, B: Unpin + Stream<Item = T>> Stream for Merge<A,B> {
    type Item = T;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        match self.first.poll_next_unpin(cx) {
            Poll::Ready(Some(x)) => Poll::Ready(Some(x)),
            Poll::Ready(None) => self.last.poll_next_unpin(cx),
            Poll::Pending => match self.last.poll_next_unpin(cx) {
                Poll::Ready(Some(x)) => Poll::Ready(Some(x)),
                _ => Poll::Pending
            }
        }
    }
}

// Stream with only unique values
pub struct Unique<S: Stream> {
    set: HashSet<Arc<S::Item>>,
    stream: S
}

impl<S: Unpin + Stream> Stream for Unique<S> where S::Item: Hash + Eq {
    type Item = Arc<S::Item>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let poll = self.stream.poll_next_unpin(cx);

        match poll {
            Poll::Ready(Some(item)) => {
                let arc = Arc::new(item);
                if self.set.contains(&arc) { return Poll::Pending; }
                Poll::Ready(Some(self.set.get_or_insert(arc).clone()))
            },

            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending
        }
    }
}

/// Stream with only unique values (only available for streams of arcs)
pub struct UniqueArc<S: Stream> {
    set: HashSet<S::Item>,
    stream: S
}

impl<T: Hash + Eq, S: Unpin + Stream<Item = Arc<T>>> Stream for UniqueArc<S>  {
    type Item = Arc<T>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        match self.stream.poll_next_unpin(cx) {
            Poll::Ready(Some(item)) if !self.set.contains(&item) => {
                self.set.insert(item.clone());
                Poll::Ready(Some(item))
            },

            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(_)) => Poll::Pending,
            Poll::Pending => Poll::Pending,
            x => {
                match x {
                    Poll::Ready(Some(_)) => println!("Poll with value: {}", self.set.len()),
                    Poll::Ready(None) => println!("Empty poll"),
                    Poll::Pending => println!("Pending")
                }

                Poll::Pending
            }
        }
    }
}