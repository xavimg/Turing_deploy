use std::{pin::Pin, future::{Future}, task::{Context, Waker, RawWaker, RawWakerVTable, Poll}};

const TABLE : RawWakerVTable = RawWakerVTable::new(
    |x| RawWaker::new(x, &TABLE), 
    |_| {},
    |_| {}, 
    |_| {}
);

pub fn seq_await<F: Future> (x: Pin<&mut F>) -> Option<F::Output> {
    let raw = RawWaker::new(std::ptr::null(), &TABLE);
    let waker = &unsafe { Waker::from_raw(raw) };
    let mut seq = Context::from_waker(&waker);

    match x.poll(&mut seq) {
        Poll::Ready(value) => Some(value),
        Poll::Pending => None
    }
}

pub fn seq_await_fn <F: Future + Unpin> (x: F) -> Option<F::Output> {
    seq_await(Pin::new(&mut Box::new(x)))
}