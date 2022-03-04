use std::ops::Deref;
use crate::*;

pub type LeftRight<T> = Either<T,T>;

impl<T> LeftRight<T> {
    #[inline]
    pub fn into_inner (self) -> T {
        match self {
            Left(x) => x,
            Right(x) => x
        }
    }

    #[inline]
    pub const fn inner_ref (&self) -> &T {
        match self {
            Left(x) => x,
            Right(x) => x
        }
    }

    #[inline]
    pub fn inner_mut (&mut self) -> &mut T {
        match self {
            Left(x) => x,
            Right(x) => x
        }
    }

    #[inline]
    pub fn inner_deref (&self) -> &<T as Deref>::Target where T: Deref {
        match self {
            Left(x) => x.deref(),
            Right(x) => x.deref()
        }
    }

    #[inline]
    pub fn inner_deref_mut (&mut self) -> &mut <T as Deref>::Target where T: DerefMut {
        match self {
            Left(x) => x.deref_mut(),
            Right(x) => x.deref_mut()
        }
    }
}

impl<T> Either<T, &T::Target> where T: Deref {
    #[inline]
    pub fn as_inner_right_deref (&self) -> &T::Target {
        match self {
            Left(x) => x,
            Right(x) => x
        }
    }
}

impl<T> Either<&T::Target, T> where T: Deref {
    #[inline]
    pub fn as_inner_left_deref (&self) -> &T::Target {
        match self {
            Left(x) => x,
            Right(x) => x
        }
    }
}

impl<T> Either<T,Option<T>> {
    #[inline]
    pub fn flatten_inner_right_option (self) -> Option<T> {
        match self {
            Left(x) => Some(x),
            Right(Some(x)) => Some(x),
            _ => None
        }
    }
}

impl<T,E> Either<T,Result<T,E>> {
    #[inline]
    pub fn flatten_inner_right_result (self) -> Result<T,E> {
        match self {
            Left(x) => Ok(x),
            Right(Ok(x)) => Ok(x),
            Right(Err(e)) => Err(e)
        }
    }
}

impl<T> Either<Option<T>,T> {
    #[inline]
    pub fn flatten_inner_left_option (self) -> Option<T> {
        match self {
            Right(x) => Some(x),
            Left(Some(x)) => Some(x),
            _ => None
        }
    }
}

impl<T,E> Either<Result<T,E>,T> {
    #[inline]
    pub fn flatten_inner_left_result (self) -> Result<T,E> {
        match self {
            Right(x) => Ok(x),
            Left(Ok(x)) => Ok(x),
            Left(Err(e)) => Err(e)
        }
    }
}

impl<T> Deref for LeftRight<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner_ref()
    }
}

impl<T> DerefMut for LeftRight<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}