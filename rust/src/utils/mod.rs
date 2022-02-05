flat_mod!(logger, color, math, serdex, array_map, take_out, randx, durationx, streamx);
use std::ops::{Deref, DerefMut};

pub type LeftRight<T> = Either<T,T>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Either<L,R> {
    Left(L),
    Right(R)
}

impl<L,R> Either<L,R> {
    pub const fn is_left (&self) -> bool {
        match self {
            Self::Left(_) => true,
            _ => false
        }
    }

    pub const fn is_right (&self) -> bool {
        match self {
            Self::Right(_) => true,
            _ => false
        }
    }

    pub fn unwrap_left (self) -> L {
        match self {
            Self::Left(x) => x,
            _ => panic!("called `EitherOrAll::unwrap_left()` on a non `Left` value")
        }
    }

    pub fn unwrap_right (self) -> R {
        match self {
            Self::Right(x) => x,
            _ => panic!("called `EitherOrAll::unwrap_right()` on a non `Right` value")
        }
    }
}

impl<T> LeftRight<T> {
    pub fn into_value (self) -> T {
        match self {
            Self::Left(x) => x,
            Self::Right(x) => x
        }
    }
}

impl<T> Deref for LeftRight<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Left(x) => x,
            Self::Right(x) => x
        }
    }
}

impl<T> DerefMut for LeftRight<T> {
    fn deref_mut (&mut self) -> &mut Self::Target {
        match self {
            Self::Left(x) => x,
            Self::Right(x) => x
        }
    }
}

pub unsafe fn upgrade<T> (ptr: &T) -> &mut T {
    let ptr = ptr as *const T as *mut T;
    &mut *ptr
}