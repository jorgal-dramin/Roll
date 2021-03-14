pub trait Rollable<'v, A> {
    fn roll(&'v mut self) -> &A;
}

pub trait NumericRollable<'v>: Rollable<'v, u16> {
    fn max(&'v self) -> u16;
    fn min(&'v self) -> u16;
}