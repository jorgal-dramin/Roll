pub trait Rollable<'v, A> {
    fn roll(&'v mut self) -> &A;
}