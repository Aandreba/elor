use quote::ToTokens;
use crate::prelude::*;

impl<A: ToTokens, B: ToTokens> ToTokens for Either<A, B> {
    #[inline(always)]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Left(x) => x.to_tokens(tokens),
            Right(x) => x.to_tokens(tokens),
        }
    }
}