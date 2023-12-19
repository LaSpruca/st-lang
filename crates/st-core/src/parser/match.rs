use super::{Parser, ParserBase, ParserStep};

pub struct Match<F, P: ParserBase>
where
    F: Fn(P::Item) -> Result<(), P::Error>,
{
    pub(super) matcher: F,
    pub(super) missing: P::Error,
    pub(super) parent: P,
}

impl<F, P: ParserBase> Match<F, P>
where
    F: Fn(P::Item) -> Result<(), P::Error>,
{
    fn r#do(&self, item: Option<P::Item>) -> Result<(), P::Error> {
        item.ok_or(self.missing.clone())
            .and_then(|item| (self.matcher)(item))
    }
}

impl<F, P: ParserBase> ParserBase for Match<F, P>
where
    F: Fn(P::Item) -> Result<(), P::Error>,
{
    type Output = P::Output;
    type Item = P::Item;
    type Error = P::Error;
}

impl<F, I, E: Clone> ParserStep for Match<F, Parser<I, E>>
where
    F: Fn(I) -> Result<(), E>,
{
    fn execute(
        &self,
        mut iter: impl Iterator<Item = Self::Item>,
    ) -> Result<Self::Output, Self::Error> {
        self.r#do(iter.next())
    }
}

impl<F, P: ParserStep> ParserStep for Match<F, P>
where
    F: Fn(P::Item) -> Result<(), P::Error>,
{
    fn execute(
        &self,
        mut iter: impl Iterator<Item = Self::Item>,
    ) -> Result<Self::Output, Self::Error> {
        self.parent
            .execute(iter.by_ref())
            .and_then(|result| self.r#do(iter.next()).map(|_| result))
    }
}
