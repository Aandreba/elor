//! A generic ```Either``` type implementation, accompanied by an ecosystem of helper traits, types, and others

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(feature = "alloc", feature = "std"))]
compile_error!("`alloc` and `std` features cannot be enabled simultaneously");

mod lr;
pub use lr::*;

mod refr;
pub use refr::*;

/// Iterator extensions
pub mod iter;

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

cfg_if::cfg_if! {
    if #[cfg(feature = "macro")] {
        mod macros;
        pub use macros::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "async")] {
        /// Future/Stream extensions 
        mod future;
        pub use future::*;
    }
}

pub mod prelude {
    pub use crate::Either;
    pub use crate::Either::{Left, Right};
}

use core::{ops::{Deref, DerefMut}, fmt::Display};
use core::hint::unreachable_unchecked;
use self::Either::*;

/// Generic data type that represents either a value
/// that's of one type or another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Either<A,B> {
    Left(A),
    Right(B)
}

impl<A,B> Either<A,B> {
    /// Returns `true` if the value is a `Left`
    #[inline(always)]
    pub const fn is_left (&self) -> bool {
        matches!(self, Left(_))
    }

    /// Returns `true` if the value is a `Right`
    #[inline(always)]
    pub const fn is_right (&self) -> bool {
        matches!(self, Right(_))
    }

    /// Returns a new `Either` with a reference to the value inside
    #[inline(always)]
    pub const fn as_ref (&self) -> Either<&A, &B> {
        match self {
            Left(x) => Either::Left(x),
            Right(x) => Either::Right(x)
        }
    }

    /// Returns a new `Either` with a mutable reference to the value inside
    #[inline(always)]
    pub fn as_mut (&mut self) -> Either<&mut A, &mut B> {
        match self {
            Left(x) => Left(x),
            Right(x) => Right(x)
        }
    }

    /// ## Example ##
    /// ```rust
    /// use elor::Either::{self, *};
    /// 
    /// fn main () {
    ///     let alpha : Either<String, Vec<u8>> = Left("hello world".to_string());
    ///     let beta : Either<&str, &[u8]> = alpha.as_deref();
    ///     assert_eq!(beta, Left("hello world"))
    /// }
    /// ```
    #[inline(always)]
    pub fn as_deref (&self) -> Either<&<A as Deref>::Target, &<B as Deref>::Target> where A: Deref, B: Deref {
        match self {
            Left(x) => Left(x.deref()),
            Right(x) => Right(x.deref())
        }
    }

    /// ## Example ##
    /// ```rust
    /// use elor::Either::{self, *};
    /// 
    /// fn main () {
    ///     let mut alpha : Either<String, Vec<u8>> = Left("hello world".to_string());
    ///     let beta : Either<&mut str, &mut [u8]> = alpha.as_deref_mut();
    ///     assert_eq!(beta, Left("hello world"))
    /// }
    /// ```
    #[inline(always)]
    pub fn as_deref_mut (&mut self) -> Either<&mut <A as Deref>::Target, &mut <B as Deref>::Target> where A: DerefMut, B: DerefMut {
        match self {
            Left(x) => Left(x.deref_mut()),
            Right(x) => Right(x.deref_mut())
        }
    }

    /// Returns a new ```Either``` with a clone of the value inside
    #[inline(always)]
    pub fn cloned (&self) -> Either<A,B> where A: Clone, B: Clone {
        match self {
            Left(x) => Left(x.clone()),
            Right(x) => Right(x.clone())
        }
    }

    /// Returns a new ```Either``` with a Copy of the value inside
    #[inline(always)]
    pub fn copied (&self) -> Either<A,B> where A: Copy, B: Copy {
        match self {
            Left(x) => Left(*x),
            Right(x) => Right(*x)
        }
    }

    /// Converts a single ```Either``` into two ```Option```
    #[inline(always)]
    pub fn unzip (self) -> (Option<A>, Option<B>) {
        match self {
            Left(x) => (Some(x), None),
            Right(x) => (None, Some(x))
        }
    }

    /// Returns the left value, panicking with a custom message if the value is on the right
    #[inline(always)]
    pub fn expect_left (self, msg: &str) -> A {
        match self {
            Left(x) => x,
            _ => panic!("{msg}")
        }
    }

    /// Returns the right value, panicking with a custom message if the value is on the left
    #[inline(always)]
    pub fn expect_right (self, msg: &str) -> B {
        match self {
            Right(x) => x,
            _ => panic!("{msg}")
        }
    }

    /// Returns the left value, ```None``` otherwise
    #[inline(always)]
    pub fn left (self) -> Option<A> {
        match self {
            Left(x) => Some(x),
            _ => None
        }
    }

    /// Returns the right value, ```None``` otherwise
    #[inline(always)]
    pub fn right (self) -> Option<B> {
        match self {
            Right(x) => Some(x),
            _ => None
        }
    }

    /// Returns `Ok` if the value is on the left, otherwise returning an `Err` with the right-side value.
    #[inline(always)]
    pub fn ok_left (self) -> Result<A, B> {
        match self {
            Self::Left(x) => Ok(x),
            Self::Right(e) => Err(e)
        }
    }

    /// Returns `Ok` if the value is on the right, otherwise returning an `Err` with the left-side value.
    #[inline(always)]
    pub fn ok_right (self) -> Result<B, A> {
        match self {
            Self::Right(x) => Ok(x),
            Self::Left(e) => Err(e)
        }
    }

    #[deprecated(since = "1.1.0", note = "use ```left``` instead")]
    #[inline(always)]
    pub fn some_left (self) -> Option<A> {
        self.left()
    }

    #[deprecated(since = "1.1.0", note = "use ```right``` instead")]
    #[inline(always)]
    pub fn some_right (self) -> Option<B> {
        self.right()
    }

    /// Returns the left value, panicking if the value is on the right
    #[inline(always)]
    pub fn unwrap_left (self) -> A {
        self.expect_left("called `Either::unwrap_left()` on a `Right` value")
    }

    /// Returns the right value, panicking if the value is on the left
    #[inline(always)]
    pub fn unwrap_right (self) -> A {
        self.expect_left("called `Either::unwrap_right()` on a `Left` value")
    }

    /// Returns the left value without checking if the value is on the right
    #[inline(always)]
    pub unsafe fn unwrap_left_unchecked (self) -> A {
        match self {
            Left(x) => x,
            _ => unreachable_unchecked()
        }
    }

    /// Returns the right value without checking if the value is on the left
    #[inline(always)]
    pub unsafe fn unwrap_right_unchecked (self) -> B {
        match self {
            Right(x) => x,
            _ => unreachable_unchecked()
        }
    }

    /// Returns the left value, or the specified value if the value is on the right
    #[inline(always)]
    pub fn unwrap_left_or (self, default: A) -> A {
        match self {
            Left(x) => x,
            _ => default
        }
    }

    /// Returns the right value, or the specified value if the value is on the left
    #[inline(always)]
    pub fn unwrap_right_or (self, default: B) -> B {
        match self {
            Right(x) => x,
            _ => default
        }
    }

    /// Returns the left value, or calls the specified function if the value is on the right
    #[inline(always)]
    pub fn unwrap_left_or_else<F: FnOnce() -> A> (self, f: F) -> A {
        match self {
            Left(x) => x,
            _ => f()
        }
    }

    /// Returns the right value, or calls the specified function if the value is on the left
    #[inline(always)]
    pub fn unwrap_right_or_else<F: FnOnce() -> B> (self, f: F) -> B {
        match self {
            Right(x) => x,
            _ => f()
        }
    }

    #[inline(always)]
    pub fn map <X, Y, F: FnOnce(A) -> X, G: FnOnce(B) -> Y> (self, f: F, g: G) -> Either<X,Y> {
        match self {
            Left(x) => Left(f(x)),
            Right(x) => Right(g(x))
        }
    }

    #[inline(always)]
    pub fn map_left<T, F: FnOnce(A) -> T> (self, f: F) -> Either<T,B> {
        match self {
            Left(x) => Left(f(x)),
            Right(x) => Right(x)
        }
    }

    #[inline(always)]
    pub fn map_right<T, F: FnOnce(B) -> T> (self, f: F) -> Either<A,T> {
        match self {
            Left(x) => Left(x),
            Right(x) => Right(f(x))
        }
    }

    #[inline(always)]
    pub fn fold<T, F: FnOnce(A) -> T, G: FnOnce(B) -> T> (self, f: F, g: G) -> T {
        match self {
            Left(x) => f(x),
            Right(x) => g(x)
        }
    }

    #[inline(always)]
    pub fn fold_left<F: FnOnce(B) -> A> (self, f: F) -> A {
        match self {
            Left(x) => x,
            Right(x) => f(x)
        }
    }

    #[inline(always)]
    pub fn fold_right<F: FnOnce(A) -> B> (self, f: F) -> B {
        match self {
            Right(x) => x,
            Left(x) => f(x)
        }
    }

    #[inline(always)]
    pub fn inverse (self) -> Either<B, A> {
        match self {
            Left(x) => Right(x),
            Right(x) => Left(x)
        }
    }
}

impl<A,B> Either<Either<A,B>,B> {
    #[inline(always)]
    pub fn flatten_left_right (self) -> Either<A,B> {
        match self {
            Left(Left(x)) => Left(x),
            Left(Right(x)) => Right(x),
            Right(x) => Right(x) 
        }
    }
}

impl<A,B> Either<A,Either<A,B>> {
    #[inline(always)]
    pub fn flatten_right_left (self) -> Either<A,B> {
        match self {
            Right(Left(x)) => Left(x),
            Right(Right(x)) => Right(x),
            Left(x) => Left(x)
        }
    }
}

impl<A,B> Either<Either<A,B>,A> {
    #[inline(always)]
    pub fn flatten_left_left (self) -> Either<A,B> {
        match self {
            Left(Left(x)) => Left(x),
            Left(Right(x)) => Right(x),
            Right(x) => Left(x) 
        }
    }
}

impl<A,B> Either<A,Either<B,A>> {
    #[inline(always)]
    pub fn flatten_right_right (self) -> Either<A,B> {
        match self {
            Right(Left(x)) => Right(x),
            Right(Right(x)) => Left(x),
            Left(x) => Left(x)
        }
    }
}

impl<A,B,EA,EB> Either<Result<A,EA>, Result<B,EB>> {
    #[inline(always)]
    pub fn flatten_result (self) -> Result<Either<A,B>, Either<EA,EB>> {
        match self {
            Left(Ok(x)) => Ok(Left(x)),
            Left(Err(e)) => Err(Left(e)),
            Right(Ok(x)) => Ok(Right(x)),
            Right(Err(e)) => Err(Right(e)) 
        }
    }

    #[inline(always)]
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
    #[inline(always)]
    pub fn flatten_option (self) -> Option<Either<A,B>> {
        match self {
            Left(Some(x)) => Some(Left(x)),
            Right(Some(x)) => Some(Right(x)),
            _ => None
        }
    }
}

impl<A: Deref, B: Deref<Target = A::Target>> Deref for Either<A,B> {
    type Target = A::Target;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        match self {
            Left(x) => x.deref(),
            Right(x) => x.deref()
        }
    }
}

impl<A: DerefMut, B: DerefMut<Target = A::Target>> DerefMut for Either<A,B> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Left(x) => x.deref_mut(),
            Right(x) => x.deref_mut()
        }
    }
}

impl<T> From<Option<T>> for Either<T,()> {
    #[inline(always)]
    fn from(x: Option<T>) -> Self {
        match x {
            Some(x) => Left(x),
            None => Right(())
        }
    }
}

impl<T> Into<Option<T>> for Either<T,()> {
    #[inline(always)]
    fn into(self) -> Option<T> {
        match self {
            Left(x) => Some(x),
            _ => None
        }
    }
}

impl<T,E> From<Result<T,E>> for Either<T,E> {
    #[inline(always)]
    fn from(x: Result<T,E>) -> Self {
        match x {
            Ok(x) => Left(x),
            Err(e) => Right(e)
        }
    }
}

impl<T,E> Into<Result<T,E>> for Either<T,E> {
    #[inline(always)]
    fn into(self) -> Result<T,E> {
        match self {
            Left(x) => Ok(x),
            Right(e) => Err(e)
        }
    }
}

impl<A,B> Display for Either<A,B> where A: Display, B: Display {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Left(x) => x.fmt(f),
            Right(x) => x.fmt(f)
        }
    }
}