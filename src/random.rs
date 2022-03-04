use rand::{prelude::Distribution, distributions::{Standard, Alphanumeric, Bernoulli, Open01, Slice, WeightedIndex, uniform::SampleUniform}};
use crate::*;

macro_rules! impl_rand {
    ($($i:ident),+) => {
        $(
            impl<A,B> Distribution<Either<A,B>> for $i where $i: Distribution<A> + Distribution<B> {
                #[inline]
                fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Either<A,B> {
                    if <Standard as Distribution<bool>>::sample(&Standard, rng) {
                        return Left(<Self as Distribution<A>>::sample(&self, rng))
                    }
            
                    Right(<Self as Distribution<B>>::sample(&self, rng))
                }
            }
        )*
    };
}

impl_rand!(
    Alphanumeric,
    Bernoulli,
    Open01
);

impl<A,B> Distribution<Either<A,B>> for Standard where Standard: Distribution<A> + Distribution<B> {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Either<A,B> {
        if <Self as Distribution<bool>>::sample(&self, rng) {
            return Left(<Self as Distribution<A>>::sample(&self, rng))
        }

        Right(<Self as Distribution<B>>::sample(&self, rng))
    }
}

impl<'a,X,A,B> Distribution<Either<A,B>> for Slice<'a, X> where Slice<'a, X>: Distribution<A> + Distribution<B> {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Either<A,B> {
        if <Standard as Distribution<bool>>::sample(&Standard, rng) {
            return Left(<Self as Distribution<A>>::sample(&self, rng))
        }

        Right(<Self as Distribution<B>>::sample(&self, rng))
    }
}

impl<X: SampleUniform + PartialOrd, A, B> Distribution<Either<A,B>> for WeightedIndex<X> where WeightedIndex<X>: Distribution<A> + Distribution<B> {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Either<A,B> {
        if <Standard as Distribution<bool>>::sample(&Standard, rng) {
            return Left(<Self as Distribution<A>>::sample(&self, rng))
        }

        Right(<Self as Distribution<B>>::sample(&self, rng))
    }
}

impl<T,A,B> Distribution<T> for Either<A,B> where A: Distribution<T>, B: Distribution<T> {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> T {
        match self {
            Left(x) => x.sample(rng),
            Right(x) => x.sample(rng)
        }
    }
} 