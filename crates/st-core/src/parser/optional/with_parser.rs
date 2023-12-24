use super::*;

impl<F, P, I, E> ParserBase for Optional<bool, F, P, (), Parser<I, E>>
where
    F: Fn(&I) -> bool,
    P: ParserStep<Item = I, Error = E>,
    E: Clone,
{
    type Output = Option<P::Output>;
    type Item = I;
    type Error = E;
}

impl<F, P, I, E> ParserStep for Optional<bool, F, P, (), Parser<I, E>>
where
    F: Fn(&I) -> bool,
    P: ParserStep<Item = I, Error = E>,
    E: Clone,
{
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error> {
        let mut iter = iter.peekable();
        let peeked = iter.peek();
        if let Some(value) = peeked {
            if (self.test)(value) {
                return self.parser.execute(iter).map(|res2| Some(res2));
            }
        }
        Ok(None)
    }
}

impl<F, P, I, E, T, TO> ParserBase for Optional<bool, F, P, T, Parser<I, E>>
where
    F: Fn(&I) -> bool,
    P: ParserStep<Item = I, Error = E>,
    T: Fn(P::Output) -> TO,
    E: Clone,
{
    type Output = Option<TO>;
    type Item = I;
    type Error = E;
}

impl<F, P, I, E, T, TO> ParserStep for Optional<bool, F, P, T, Parser<I, E>>
where
    F: Fn(&I) -> bool,
    P: ParserStep<Item = I, Error = E>,
    T: Fn(P::Output) -> TO,
    E: Clone,
{
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error> {
        let mut iter = iter.peekable();
        let peeked = iter.peek();
        if let Some(value) = peeked {
            if (self.test)(value) {
                return self
                    .parser
                    .execute(iter)
                    .map(|res2| Some((self.transform)(res2)));
            }
        }
        Ok(None)
    }
}

impl<I, F, P, Item, E> ParserBase for Optional<Option<I>, F, P, (), Parser<Item, E>>
where
    F: Fn(&Item) -> Option<I>,
    P: ParserStep<Item = Item, Error = E>,
    E: Clone,
{
    type Output = Option<(I, P::Output)>;
    type Item = Item;
    type Error = E;
}

impl<I, F, P, Item, E> ParserStep for Optional<Option<I>, F, P, (), Parser<Item, E>>
where
    F: Fn(&Item) -> Option<I>,
    P: ParserStep<Item = Item, Error = E>,
    E: Clone,
{
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error> {
        let mut iter = iter.peekable();
        let peeked = iter.peek();
        if let Some(value) = peeked {
            if let Some(tested) = (self.test)(value) {
                return self.parser.execute(iter).map(|out| Some((tested, out)));
            }
        }
        Ok(None)
    }
}

impl<I, F, P, Item, E, T, TO> ParserBase for Optional<Option<I>, F, P, T, Parser<Item, E>>
where
    F: Fn(&Item) -> Option<I>,
    P: ParserStep<Item = Item, Error = E>,
    T: Fn(I, P::Output) -> TO,
    E: Clone,
{
    type Output = Option<TO>;
    type Item = Item;
    type Error = E;
}

impl<I, F, P, Item, E, T, TO> ParserStep for Optional<Option<I>, F, P, T, Parser<Item, E>>
where
    F: Fn(&Item) -> Option<I>,
    P: ParserStep<Item = Item, Error = E>,
    T: Fn(I, P::Output) -> TO,
    E: Clone,
{
    fn execute(&self, iter: impl Iterator<Item = Self::Item>) -> Result<Self::Output, Self::Error> {
        let mut iter = iter.peekable();
        let peeked = iter.peek();
        if let Some(value) = peeked {
            if let Some(tested) = (self.test)(value) {
                return self
                    .parser
                    .execute(iter)
                    .map(|out| Some((self.transform)(tested, out)));
            }
        }
        Ok(None)
    }
}
