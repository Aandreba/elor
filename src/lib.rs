mod lr;
pub use lr::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "serialize")] {
        mod serde;
        pub use self::serde::*;
        use ::serde::{Serialize, Deserialize};
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "random")] {
        mod random;
        pub use random::*;
    }
}

use std::{ops::{Deref, DerefMut}, fmt::Display};
use self::Either::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Either<A,B> {
    Left(A),
    Right(B)
}

impl<A,B> Either<A,B> {
    #[inline]
    pub const fn is_left (&self) -> bool {
        matches!(self, Left(_))
    }

    #[inline]
    pub const fn is_right (&self) -> bool {
        matches!(self, Right(_))
    }

    #[inline]
    pub const fn as_ref (&self) -> Either<&A, &B> {
        match self {
            Left(x) => Either::Left(x),
            Right(x) => Either::Right(x)
        }
    }

    #[inline]
    pub fn as_mut (&mut self) -> Either<&mut A, &mut B> {
        match self {
            Left(x) => Left(x),
            Right(x) => Right(x)
        }
    }

    /// ## Example ##
    /// ```rust
    /// use etot::Either::{self, *};
    /// 
    /// fn main () {
    ///     let alpha : Either<String, Vec<u8>> = Left("hello world".to_string());
    ///     let beta : Either<&str, &[u8]> = alpha.as_deref();
    ///     assert_eq!(beta, Left("hello world"))
    /// }
    /// ```
    #[inline]
    pub fn as_deref (&self) -> Either<&<A as Deref>::Target, &<B as Deref>::Target> where A: Deref, B: Deref{
        match self {
            Left(x) => Left(x.deref()),
            Right(x) => Right(x.deref())
        }
    }

    /// ## Example ##
    /// ```rust
    /// use etot::Either::{self, *};
    /// 
    /// fn main () {
    ///     let mut alpha : Either<String, Vec<u8>> = Left("hello world".to_string());
    ///     let beta : Either<&mut str, &mut [u8]> = alpha.as_deref_mut();
    ///     assert_eq!(beta, Left("hello world"))
    /// }
    /// ```
    #[inline]
    pub fn as_deref_mut (&mut self) -> Either<&mut <A as Deref>::Target, &mut <B as Deref>::Target> where A: DerefMut, B: DerefMut {
        match self {
            Left(x) => Left(x.deref_mut()),
            Right(x) => Right(x.deref_mut())
        }
    }

    #[inline]
    pub fn cloned (&self) -> Either<A,B> where A: Clone, B: Clone {
        match self {
            Left(x) => Left(x.clone()),
            Right(x) => Right(x.clone())
        }
    }

    #[inline]
    pub fn unzip (self) -> (Option<A>, Option<B>) {
        match self {
            Left(x) => (Some(x), None),
            Right(x) => (None, Some(x))
        }
    }

    #[inline]
    pub fn expect_left (self, msg: &str) -> A {
        match self {
            Left(x) => x,
            _ => panic!("{msg}")
        }
    }

    #[inline]
    pub fn expect_right (self, msg: &str) -> B {
        match self {
            Right(x) => x,
            _ => panic!("{msg}")
        }
    }

    #[inline]
    pub fn ok_left (self) -> Option<A> {
        match self {
            Left(x) => Some(x),
            _ => None
        }
    }

    #[inline]
    pub fn ok_right (self) -> Option<B> {
        match self {
            Right(x) => Some(x),
            _ => None
        }
    }

    #[inline]
    pub fn unwrap_left (self) -> A {
        self.expect_left("called `Either::unwrap_left()` on a `Right` value")
    }

    #[inline]
    pub fn unwrap_right (self) -> A {
        self.expect_left("called `Either::unwrap_right()` on a `Left` value")
    }

    #[inline]
    pub fn unwrap_left_or (self, default: A) -> A {
        match self {
            Left(x) => x,
            _ => default
        }
    }

    #[inline]
    pub fn unwrap_right_or (self, default: B) -> B {
        match self {
            Right(x) => x,
            _ => default
        }
    }

    #[inline]
    pub fn unwrap_left_or_else<F: FnOnce() -> A> (self, f: F) -> A {
        match self {
            Left(x) => x,
            _ => f()
        }
    }

    #[inline]
    pub fn unwrap_right_or_else<F: FnOnce() -> B> (self, f: F) -> B {
        match self {
            Right(x) => x,
            _ => f()
        }
    }

    #[inline]
    pub fn fold<T, FA: FnOnce(A) -> T, FB: FnOnce(B) -> T> (self, fa: FA, fb: FB) -> T {
        match self {
            Left(x) => fa(x),
            Right(x) => fb(x)
        }
    }

    #[inline]
    pub fn fold_left<F: FnOnce(B) -> A> (self, f: F) -> A {
        match self {
            Left(x) => x,
            Right(x) => f(x)
        }
    }

    #[inline]
    pub fn fold_right<F: FnOnce(A) -> B> (self, f: F) -> B {
        match self {
            Right(x) => x,
            Left(x) => f(x)
        }
    }

    #[inline]
    pub fn inverse (self) -> Either<B, A> {
        match self {
            Left(x) => Right(x),
            Right(x) => Left(x)
        }
    }
}

impl<A,B> Either<Either<A,B>,B> {
    #[inline]
    pub fn flatten_left_right (self) -> Either<A,B> {
        match self {
            Left(Left(x)) => Left(x),
            Left(Right(x)) => Right(x),
            Right(x) => Right(x) 
        }
    }
}

impl<A,B> Either<A,Either<A,B>> {
    #[inline]
    pub fn flatten_right_left (self) -> Either<A,B> {
        match self {
            Right(Left(x)) => Left(x),
            Right(Right(x)) => Right(x),
            Left(x) => Left(x)
        }
    }
}

impl<A,B> Either<Either<A,B>,A> {
    #[inline]
    pub fn flatten_left_left (self) -> Either<A,B> {
        match self {
            Left(Left(x)) => Left(x),
            Left(Right(x)) => Right(x),
            Right(x) => Left(x) 
        }
    }
}

impl<A,B> Either<A,Either<B,A>> {
    #[inline]
    pub fn flatten_right_right (self) -> Either<A,B> {
        match self {
            Right(Left(x)) => Right(x),
            Right(Right(x)) => Left(x),
            Left(x) => Left(x)
        }
    }
}

impl<A,B,EA,EB> Either<Result<A,EA>, Result<B,EB>> {
    #[inline]
    pub fn flatten_result (self) -> Result<Either<A,B>, Either<EA,EB>> {
        match self {
            Left(Ok(x)) => Ok(Left(x)),
            Left(Err(e)) => Err(Left(e)),
            Right(Ok(x)) => Ok(Right(x)),
            Right(Err(e)) => Err(Right(e)) 
        }
    }

    #[inline]
    pub fn expand_result (result: Result<Either<A,B>, Either<EA,EB>>) -> Self {
        match result {
            Ok(Left(x)) => Left(Ok(x)),
            Ok(Right(x)) => Right(Ok(x)),
            Err(Left(e)) => Left(Err(e)),
            Err(Right(e)) => Right(Err(e)) 
        }
    }
}

impl<A,B> Either<Option<A>, Option<B>> {
    #[inline]
    pub fn flatten_option (self) -> Option<Either<A,B>> {
        match self {
            Left(Some(x)) => Some(Left(x)),
            Right(Some(x)) => Some(Right(x)),
            _ => None
        }
    }
}

impl<T> From<Option<T>> for Either<T,()> {
    #[inline]
    fn from(x: Option<T>) -> Self {
        match x {
            Some(x) => Left(x),
            None => Right(())
        }
    }
}

impl<T> Into<Option<T>> for Either<T,()> {
    #[inline]
    fn into(self) -> Option<T> {
        match self {
            Left(x) => Some(x),
            _ => None
        }
    }
}

impl<T,E> From<Result<T,E>> for Either<T,E> {
    #[inline]
    fn from(x: Result<T,E>) -> Self {
        match x {
            Ok(x) => Left(x),
            Err(e) => Right(e)
        }
    }
}

impl<T,E> Into<Result<T,E>> for Either<T,E> {
    #[inline]
    fn into(self) -> Result<T,E> {
        match self {
            Left(x) => Ok(x),
            Right(e) => Err(e)
        }
    }
}

impl<A,B> Display for Either<A,B> where A: Display, B: Display {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Left(x) => x.fmt(f),
            Right(x) => x.fmt(f)
        }
    }
}