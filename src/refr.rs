use crate::prelude::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::borrow::{Cow, ToOwned};
        use std::boxed::Box;
    } else if #[cfg(feature = "alloc")] {
        extern crate alloc;
        use alloc::borrow::{Cow, ToOwned};
        use alloc::boxed::Box;
    }
}

/// Borrowed or owned
pub type Boo<'a, T> = Either<&'a T, T>;

impl<'a, T> Boo<'a, T> {
    #[inline(always)]
    pub fn as_inner_ref (&self) -> &T {
        match self {
            Left(x) => *x,
            Right(x) => x
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[cfg(any(feature = "std", feature = "alloc"))]
impl<A, B> Either<Box<A>, Box<B>> {
    /// Convert into a single boxed value
    #[inline(always)]
    pub fn into_boxed (self) -> Box<Either<A, B>> {
        match self {
            Left(x) => Box::new(Left(*x)),
            Right(x) => Box::new(Right(*x)),
        }
    }
}

impl<T: ?Sized, A: AsRef<T>, B: AsRef<T>> AsRef<T> for Either<A, B> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        match self {
            Left(x) => x.as_ref(),
            Right(x) => x.as_ref(),
        }
    }
}

impl<T: ?Sized, A: AsMut<T>, B: AsMut<T>> AsMut<T> for Either<A, B> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        match self {
            Left(x) => x.as_mut(),
            Right(x) => x.as_mut(),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, T: ToOwned> From<Cow<'a, T>> for Either<&'a T, <T as ToOwned>::Owned> {
    #[inline(always)]
    fn from(x: Cow<'a, T>) -> Self {
        match x {
            Cow::Borrowed(x) => Left(x),
            Cow::Owned(x) => Right(x),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, T: ToOwned> Into<Cow<'a, T>> for Either<&'a T, T::Owned> {
    #[inline(always)]
    fn into(self) -> Cow<'a, T> {
        match self {
            Left(x) => Cow::Borrowed(x),
            Right(x) => Cow::Owned(x),
        }
    }
}