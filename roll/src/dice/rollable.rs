pub trait Rollable<A> {
    fn roll(& mut self) -> &A;
}

pub trait NumericRollable: Rollable<u16> {
    fn max(&self) -> u16;
    fn min(&self) -> u16;
}