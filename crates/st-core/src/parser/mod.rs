pub mod r#match;
pub mod optional;
pub mod parser_macro;
pub mod take;
pub mod then;
pub mod transform;

use crate::{error::CompileError, tokenizer::TokenEnum};
use std::marker::PhantomData;

use self::{optional::Optional, r#match::Match, take::Take, then::Then, transform::Transform};

pub fn parse(mut _iter: impl Iterator<Item = TokenEnum>) -> Result<(), CompileError> {
    todo!()
}

pub trait ParserBase: Sized
where
    Self::Error: Clone,
{
    type Output;
    type Item;
    type Error;

    fn r#match<F>(self, matcher: F, missing: Self::Error) -> Match<F, Self>
    where
        F: Fn(Self::Item) -> Result<(), Self::Error>,
    {
        Match {
            matcher,
            missing,
            parent: self,
        }
    }

    fn take<I, F>(self, matcher: F, missing: Self::Error) -> Take<I, F, Self>
    where
        F: Fn(Self::Item) -> Result<I, Self::Error>,
    {
        Take {
            matcher,
            missing,
            parent: self,
        }
    }

    fn transform<I, F>(self, transformer: F) -> Transform<I, F, Self>
    where
        F: Fn(Self::Output) -> I,
        Self: ParserStep,
    {
        Transform {
            transformer,
            parent: self,
        }
    }

    fn then<T>(self, parser: T) -> Then<T, Self>
    where
        T: ParserStep<Item = Self::Item, Error = Self::Error>,
    {
        Then {
            parser,
            parent: self,
        }
    }

    fn optional<I, F, P>(self, test: F, parser: P) -> Optional<I, F, P, (), Self>
    where
        F: Fn(&Self::Item) -> I,
        P: ParserStep<Item = Self::Item, Error = Self::Error>,
    {
        Optional {
            test,
            parser,
            transform: (),
            parent: self,
        }
    }

    fn optional_take_transform<I, F, P, T, TO>(
        self,
        test: F,
        parser: P,
        transform: T,
    ) -> Optional<Option<I>, F, P, T, Self>
    where
        F: Fn(&Self::Item) -> Option<I>,
        P: ParserStep<Item = Self::Item, Error = Self::Error>,
        T: Fn(I, P::Output) -> TO,
    {
        Optional {
            test,
            parser,
            transform,
            parent: self,
        }
    }

    fn optional_match_transform<F, P, T, TO>(
        self,
        test: F,
        parser: P,
        transform: T,
    ) -> Optional<bool, F, P, T, Self>
    where
        F: Fn(&Self::Item) -> bool,
        P: ParserStep<Item = Self::Item, Error = Self::Error>,
        T: Fn(P::Output) -> TO,
    {
        Optional {
            test,
            parser,
            transform,
            parent: self,
        }
    }
}

pub trait ParserStep: ParserBase {
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error>;
}

pub struct Parser<I, E: Clone> {
    _type: PhantomData<(I, E)>,
}

impl<I, E> Parser<I, E>
where
    E: Clone,
{
    pub fn new() -> Self {
        Self { _type: PhantomData }
    }
}

impl<I, E: Clone> ParserBase for Parser<I, E> {
    type Output = ();
    type Item = I;
    type Error = E;
}

// #[cfg(test)]
mod tests;
