use super::{Parser, ParserBase, ParserStep};

pub struct Take<I, F, P>
where
    P: ParserBase,
    F: Fn(P::Item) -> Result<I, P::Error>,
{
    pub(super) matcher: F,
    pub(super) missing: P::Error,
    pub(super) parent: P,
}

impl<I, F, P> Take<I, F, P>
where
    P: ParserBase,
    F: Fn(P::Item) -> Result<I, P::Error>,
{
    fn r#do(&self, item: Option<P::Item>) -> Result<I, P::Error> {
        item.ok_or(self.missing.clone()).and_then(&self.matcher)
    }
}

impl<I, F, Item, E> ParserBase for Take<I, F, Parser<Item, E>>
where
    F: Fn(Item) -> Result<I, E>,
    E: Clone,
{
    type Output = I;
    type Item = Item;
    type Error = E;
}

impl<I, F, Item, E> ParserStep for Take<I, F, Parser<Item, E>>
where
    F: Fn(Item) -> Result<I, E>,
    E: Clone,
{
    fn execute(
        &self,
        mut iter: impl Iterator<Item = Self::Item>,
    ) -> Result<Self::Output, Self::Error> {
        self.r#do(iter.next())
    }
}

impl<I, F, P> ParserBase for Take<I, F, P>
where
    P: ParserStep,
    F: Fn(P::Item) -> Result<I, P::Error>,
{
    type Output = (P::Output, I);
    type Item = P::Item;
    type Error = P::Error;
}

impl<I, F, P> ParserStep for Take<I, F, P>
where
    P: ParserStep,
    F: Fn(P::Item) -> Result<I, P::Error>,
{
    fn execute(
        &self,
        mut iter: impl Iterator<Item = Self::Item>,
    ) -> Result<Self::Output, Self::Error> {
        self.parent
            .execute(iter.by_ref())
            .and_then(|res| self.r#do(iter.next()).map(|res2| (res, res2)))
    }
}
