use futures::{Future, FutureExt, Stream, StreamExt};
use core::ops::*;
use core::hint::unreachable_unchecked;
use crate::prelude::*;

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl<A: Future, B: Future> Into<futures::future::Either<A,B>> for Either<A,B> {
    #[inline(always)]
    fn into(self) -> futures::future::Either<A,B> {
        match self {
            Left(x) => futures::future::Either::Left(x),
            Right(x) => futures::future::Either::Right(x)
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl<A: Future, B: Future> From<futures::future::Either<A,B>> for Either<A,B> {
    #[inline(always)]
    fn from(x: futures::future::Either<A,B>) -> Self {
        match x {
            futures::future::Either::Left(x) => Left(x),
            futures::future::Either::Right(x) => Right(x),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl<A: Future + Unpin, B: Future + Unpin> Future for Either<A,B> {
    type Output = Either<A::Output, B::Output>;

    #[inline(always)]
    fn poll(mut self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        match self.deref_mut() {
            Left(x) => x.poll_unpin(cx).map(Left),
            Right(x) => x.poll_unpin(cx).map(Right),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl<A: Stream + Unpin, B: Stream + Unpin> Stream for Either<A,B> {
    type Item = Either<A::Item, B::Item>;

    #[inline(always)]
    fn poll_next(mut self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<Self::Item>> {
        match self.deref_mut() {
            Left(x) => x.poll_next_unpin(cx).map(|x| x.map(Left)),
            Right(x) => x.poll_next_unpin(cx).map(|x| x.map(Right)),
        }
    }
}