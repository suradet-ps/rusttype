macro_rules! noshrink {
    () => {
        fn simplify(&mut self) -> bool {
            false
        }
        fn complicate(&mut self) -> bool {
            false
        }
    };
}

//==============================================================================
// Just
//==============================================================================

/// A `Strategy` which always produces a single value and never
/// simplifies.
#[derive(Clone, Copy, Debug)]
#[must_use = "strategies do nothing unless used"]
pub struct Just<T: Clone + fmt::Debug>(
    /// The value produced by this strategy.
    pub T,
);

impl<T: Clone + fmt::Debug> Strategy for Just<T> {
    type Tree = Self;
    type Value = T;

    fn new_tree(&self, _: &mut TestRunner) -> NewTree<Self> {
        Ok(self.clone())
    }
}

impl<T: Clone + fmt::Debug> ValueTree for Just<T> {
    type Value = T;
    noshrink!();
    fn current(&self) -> T {
        self.0.clone()
    }
}
