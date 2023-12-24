mod with_parser;

use super::{Parser, ParserBase, ParserStep};

pub struct Optional<I, F, P, T, S>
where
    S: ParserBase,
    F: Fn(&S::Item) -> I,
    P: ParserStep<Item = S::Item, Error = S::Error>,
{
    pub(super) test: F,
    pub(super) parser: P,
    pub(super) transform: T,
    pub(super) parent: S,
}

impl<F, P, S> ParserBase for Optional<bool, F, P, (), S>
where
    S: ParserStep,
    F: Fn(&S::Item) -> bool,
    P: ParserStep<Item = S::Item, Error = S::Error>,
{
    type Output = (S::Output, Option<P::Output>);
    type Item = S::Item;
    type Error = S::Error;
}

impl<F, P, S> ParserStep for Optional<bool, F, P, (), S>
where
    S: ParserStep,
    F: Fn(&S::Item) -> bool,
    P: ParserStep<Item = S::Item, Error = S::Error>,
{
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error> {
        let mut iter = iter.peekable();
        self.parent.execute(iter.by_ref()).and_then(|res| {
            let peeked = iter.peek();
            if let Some(value) = peeked {
                if (self.test)(value) {
                    return self.parser.execute(iter).map(|res2| (res, Some(res2)));
                }
            }
            Ok((res, None))
        })
    }
}

impl<F, P, S, T, TO> ParserBase for Optional<bool, F, P, T, S>
where
    S: ParserStep,
    F: Fn(&S::Item) -> bool,
    P: ParserStep<Item = S::Item, Error = S::Error>,
    T: Fn(P::Output) -> TO,
{
    type Output = (S::Output, Option<TO>);
    type Item = S::Item;
    type Error = S::Error;
}

impl<F, P, S, T, TO> ParserStep for Optional<bool, F, P, T, S>
where
    S: ParserStep,
    F: Fn(&S::Item) -> bool,
    P: ParserStep<Item = S::Item, Error = S::Error>,
    T: Fn(P::Output) -> TO,
{
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error> {
        let mut iter = iter.peekable();
        self.parent.execute(iter.by_ref()).and_then(|res| {
            let peeked = iter.peek();
            if let Some(value) = peeked {
                if (self.test)(value) {
                    return self
                        .parser
                        .execute(iter)
                        .map(|res2| (res, Some((self.transform)(res2))));
                }
            }
            Ok((res, None))
        })
    }
}

impl<I, F, P, S> ParserBase for Optional<Option<I>, F, P, (), S>
where
    S: ParserStep,
    F: Fn(&S::Item) -> Option<I>,
    P: ParserStep<Item = S::Item, Error = S::Error>,
{
    type Output = (S::Output, Option<(I, P::Output)>);
    type Item = S::Item;
    type Error = S::Error;
}

impl<I, F, P, S> ParserStep for Optional<Option<I>, F, P, (), S>
where
    S: ParserStep,
    F: Fn(&S::Item) -> Option<I>,
    P: ParserStep<Item = S::Item, Error = S::Error>,
{
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error> {
        let mut iter = iter.peekable();
        self.parent.execute(iter.by_ref()).and_then(|res| {
            let peeked = iter.peek();
            if let Some(value) = peeked {
                if let Some(tested) = (self.test)(value) {
                    return self
                        .parser
                        .execute(iter)
                        .map(|out| (res, Some((tested, out))));
                }
            }
            Ok((res, None))
        })
    }
}

impl<I, F, P, S, T, TO> ParserBase for Optional<Option<I>, F, P, T, S>
where
    S: ParserStep,
    F: Fn(&S::Item) -> Option<I>,
    P: ParserStep<Item = S::Item, Error = S::Error>,
    T: Fn(I, P::Output) -> TO,
{
    type Output = (S::Output, Option<TO>);
    type Item = S::Item;
    type Error = S::Error;
}

impl<I, F, P, S, T, TO> ParserStep for Optional<Option<I>, F, P, T, S>
where
    S: ParserStep,
    F: Fn(&S::Item) -> Option<I>,
    P: ParserStep<Item = S::Item, Error = S::Error>,
    T: Fn(I, P::Output) -> TO,
{
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error> {
        let mut iter = iter.peekable();
        self.parent.execute(iter.by_ref()).and_then(|res| {
            let peeked = iter.peek();
            if let Some(value) = peeked {
                if let Some(tested) = (self.test)(value) {
                    return self
                        .parser
                        .execute(iter)
                        .map(|out| (res, Some((self.transform)(tested, out))));
                }
            }
            Ok((res, None))
        })
    }
}
