use std::{pin::Pin, task::Poll};
use futures::{Stream};

pub trait Streamx: Stream {
    /// Async version of ```Iterator::cloned```
    fn cloned<'a, T: Clone> (self) -> StreamCloned<'a, T, Self> where Self: Stream<Item = &'a T> + Sized {
        StreamCloned(Box::pin(self))
    }

    /// Asynchronus filter. Unlike it's ```futures``` counterpart, this stream will report itself as ```Poll::Pending```
    /// until it finds a value that mathches
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
}