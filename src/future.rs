use futures::{Future, Stream};
use crate::{EitherProj, prelude::*};

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
impl<A: Future, B: Future> Future for Either<A,B> {
    type Output = Either<A::Output, B::Output>;

    #[inline(always)]
    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        match self.project() {
            EitherProj::Left(x) => x.poll(cx).map(Left),
            EitherProj::Right(x) => x.poll(cx).map(Right),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl<A: Stream, B: Stream> Stream for Either<A,B> {
    type Item = Either<A::Item, B::Item>;

    #[inline(always)]
    fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<Self::Item>> {
        match self.project() {
            EitherProj::Left(x) => x.poll_next(cx).map(|x| x.map(Left)),
            EitherProj::Right(x) => x.poll_next(cx).map(|x| x.map(Right)),
        }
    }
}