use super::{ParserBase, ParserStep};

pub struct Transform<I, F, P>
where
    F: Fn(P::Output) -> I,
    P::Error: Clone,
    P: ParserStep,
{
    pub(super) transformer: F,
    pub(super) parent: P,
}

impl<I, F, P> ParserBase for Transform<I, F, P>
where
    F: Fn(P::Output) -> I,
    P::Error: Clone,
    P: ParserStep,
{
    type Output = I;
    type Item = P::Item;
    type Error = P::Error;
}

impl<I, F, P> ParserStep for Transform<I, F, P>
where
    F: Fn(P::Output) -> I,
    P::Error: Clone,
    P: ParserStep,
{
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error> {
        self.parent.execute(iter).map(&self.transformer)
    }
}
