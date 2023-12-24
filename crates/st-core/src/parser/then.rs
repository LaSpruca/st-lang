use super::{Parser, ParserBase, ParserStep};

pub struct Then<T, P>
where
    T: ParserStep<Item = P::Item, Error = P::Error>,
    P: ParserBase,
{
    pub(super) parent: P,
    pub(super) parser: T,
}

impl<T, I, E> ParserBase for Then<T, Parser<I, E>>
where
    T: ParserStep<Item = I, Error = E>,
    E: Clone,
{
    type Item = I;
    type Error = E;
    type Output = T::Output;
}

impl<T, I, E> ParserStep for Then<T, Parser<I, E>>
where
    T: ParserStep<Item = I, Error = E>,
    E: Clone,
{
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error> {
        self.parser.execute(iter)
    }
}

impl<T, P> ParserBase for Then<T, P>
where
    T: ParserStep<Item = P::Item, Error = P::Error>,
    P: ParserStep,
{
    type Item = P::Item;
    type Error = P::Error;
    type Output = (P::Output, T::Output);
}

impl<T, P> ParserStep for Then<T, P>
where
    T: ParserStep<Item = P::Item, Error = P::Error>,
    P: ParserStep,
{
    fn execute(
        &self,
        mut iter: impl Iterator<Item = Self::Item>,
    ) -> Result<Self::Output, Self::Error> {
        self.parent
            .execute(iter.by_ref())
            .and_then(|first| self.parser.execute(iter).map(|second| (first, second)))
    }
}
