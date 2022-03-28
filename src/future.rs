use futures::{Future, FutureExt, Stream, StreamExt};
use crate::*;

impl<A: Future, B: Future> Into<futures::future::Either<A,B>> for Either<A,B> {
    #[inline]
    fn into(self) -> futures::future::Either<A,B> {
        match self {
            Left(x) => futures::future::Either::Left(x),
            Right(x) => futures::future::Either::Right(x)
        }
    }
}

impl<A: Future, B: Future> From<futures::future::Either<A,B>> for Either<A,B> {
    #[inline]
    fn from(x: futures::future::Either<A,B>) -> Self {
        match x {
            futures::future::Either::Left(x) => Left(x),
            futures::future::Either::Right(x) => Right(x),
        }
    }
}

impl<A: Future + Unpin, B: Future + Unpin> Future for Either<A,B> {
    type Output = Either<A::Output, B::Output>;

    #[inline]
    fn poll(mut self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        match self.deref_mut() {
            Left(x) => x.poll_unpin(cx).map(|x| Left(x)),
            Right(x) => x.poll_unpin(cx).map(|x| Right(x)),
        }
    }
}

impl<A: Stream + Unpin, B: Stream + Unpin> Stream for Either<A,B> {
    type Item = Either<A::Item, B::Item>;

    #[inline]
    fn poll_next(mut self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<Self::Item>> {
        match self.deref_mut() {
            Left(x) => x.poll_next_unpin(cx).map(|x| x.map(|x| Left(x))),
            Right(x) => x.poll_next_unpin(cx).map(|x| x.map(|x| Right(x))),
        }
    }
}