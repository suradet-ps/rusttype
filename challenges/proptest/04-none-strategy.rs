#[must_use = "strategies do nothing unless used"]
struct NoneStrategy<T>(PhantomData<T>);
impl<T> Clone for NoneStrategy<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for NoneStrategy<T> {}
impl<T> fmt::Debug for NoneStrategy<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NoneStrategy")
    }
}
impl<T: fmt::Debug> Strategy for NoneStrategy<T> {
    type Tree = Self;
    type Value = Option<T>;

    fn new_tree(&self, _: &mut TestRunner) -> NewTree<Self> {
        Ok(*self)
    }
}
impl<T: fmt::Debug> ValueTree for NoneStrategy<T> {
    type Value = Option<T>;

    fn current(&self) -> Option<T> {
        None
    }
    fn simplify(&mut self) -> bool {
        false
    }
    fn complicate(&mut self) -> bool {
        false
    }
}
