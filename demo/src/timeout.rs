use std::{time::Duration, task::Poll};
use futures::{Future, FutureExt};
use tokio::time::Instant;

pub struct Timeout<F> {
    future: F,
    left: Duration,
    last_date: Option<Instant>
}

impl<F: Unpin + Future> Timeout<F> {
    #[inline]
    pub fn new (future: F, timeout: Duration) -> Self {
        Self { future, left: timeout, last_date: None }
    }
}

impl<F: Unpin + Future> Future for Timeout<F> {
    type Output = Option<F::Output>;

    #[inline]
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Poll::Ready(value) = self.future.poll_unpin(cx) {
            return Poll::Ready(Some(value))
        }

        let now = Instant::now();
        if let Some(last) = self.last_date {
            let delta = now - last;
            if self.left <= delta { return Poll::Ready(None); }
            self.left -= delta;
        }

        self.last_date = Some(now);
        Poll::Pending
    }
}