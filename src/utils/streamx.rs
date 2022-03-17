use std::{pin::Pin, task::Poll};
use futures::{Stream, Future, FutureExt, StreamExt};
use tokio::task::{JoinHandle, JoinError};
use crate::Either;

pub trait Streamx: Stream {
    /// Async version of ```Iterator::cloned```
    #[inline]
    fn cloned<'a, T: Clone> (self) -> StreamCloned<'a, T, Self> where Self: Stream<Item = &'a T> + Sized {
        StreamCloned(Box::pin(self))
    }

    /// Merges results from two streams into one unorderedly
    #[inline]
    fn merge<S: Stream<Item = Self::Item>> (self, other: S) -> Merge<Self, S> where Self: Sized {
        Merge { first: self, last: other }
    }

    /// Asynchronus filter. Unlike it's ```futures``` counterpart, this stream will report itself as ```Poll::Pending```
    /// until it finds a value that mathches
    #[inline]
    fn async_filter<F: Fn(&Self::Item) -> bool> (self, predicate: F) -> AsyncFilter<Self, F> where Self: Sized {
        AsyncFilter {
            stream: Box::pin(self),
            filter: predicate
        }
    }
}

impl<T: Stream> Streamx for T {}

pub struct AsyncFilter<S: Stream, F: Fn(&S::Item) -> bool> {
    stream: Pin<Box<S>>,
    filter: F
}

impl<S: Stream, F: Fn(&S::Item) -> bool> Stream for AsyncFilter<S, F> where Self: Unpin {
    type Item = S::Item;

    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        if let Poll::Ready(result) = self.stream.as_mut().poll_next(cx) {
            if let Some(value) = result {
                if (self.filter)(&value) { return Poll::Ready(Some(value)) }
                return Poll::Pending
            }

            return Poll::Ready(None)
        }

        Poll::Pending
    }
}

pub struct StreamCloned<'a, T: 'a + Clone, S: Stream<Item = &'a T>>(Pin<Box<S>>);

impl<'a, T: 'a + Clone, S: Stream<Item = &'a T>> Stream for StreamCloned<'a,T,S> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(result) = self.0.as_mut().poll_next(cx) {
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