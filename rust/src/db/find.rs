use std::{pin::Pin, task::Poll};
use futures::{Future};
use crate::{cache::{CollectionCache}, Either};

struct FindFuture<'a, E1, E2, T1, T2, L: Future<Output = Result<T1,E1>>, R: Future<Output = Result<T2,E2>>> {
    left: Pin<&'a mut L>,
    right: Pin<&'a mut R>,
}

impl<'a, E1, E2, T1, T2, L: Future<Output = Result<T1,E1>>, R: Future<Output = Result<T2,E2>>> Future for FindFuture<'a, E1, E2, T1, T2, L, R> {
    type Output = Result<Either<T1,T2>, (E1, E2)>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let poll1 = self.left.as_mut().poll(cx);
        let poll2 = self.right.as_mut().poll(cx);

        if let Poll::Ready(result1) = poll1 {
            return match result1 {
                Err(e1) => match poll2 {
                    Poll::Ready(result2) => match result2 {
                        Err(e2) => Poll::Ready(Err((e1, e2))),
                        Ok(x) => Poll::Ready(Ok(Either::Right(x)))
                    },

                    Poll::Pending => Poll::Pending
                },

                Ok(x) => Poll::Ready(Ok(Either::Left(x)))
            }
        } else if let Poll::Ready(result2) = poll2 {
            return match result2 {
                Ok(x) => Poll::Ready(Ok(Either::Right(x))),
                Err(_) => Poll::Pending
            }
        }

        Poll::Pending
    }
}

impl<T> CollectionCache<T> {
    /// Start both futures simultaneously. If the one that finishes first has no errors, return it's value.
    /// Otherwise, let the other end and return it's value
    pub(crate) async fn any_of<U, V, E1, E2, A: Future<Output = Result<U,E1>>, B: Future<Output = Result<V,E2>>> (mut first: Pin<Box<A>>, mut last: Pin<Box<B>>) -> Result<Either<U,V>, (E1, E2)> {
        let join = FindFuture {
            left: first.as_mut(),
            right: last.as_mut()
        };

        join.await
    }
}