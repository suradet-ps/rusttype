impl Probability {
    /// Creates a `Probability` from a `f64`.
    ///
    /// # Panics
    ///
    /// Panics if the probability is outside interval `[0.0, 1.0]`.
    pub fn new(prob: f64) -> Self {
        assert!(prob >= 0.0 && prob <= 1.0);
        Probability(prob)
    }

    // Don't rely on these existing internally:

    /// Merges self together with some other argument producing a product
    /// type expected by some implementations of `A: Arbitrary` in
    /// `A::Parameters`. This can be more ergonomic to work with and may
    /// help type inference.
    pub fn with<X>(self, and: X) -> product_type![Self, X] {
        product_pack![self, and]
    }

    /// Merges self together with some other argument generated with a
    /// default value producing a product type expected by some
    /// implementations of `A: Arbitrary` in `A::Parameters`.
    /// This can be more ergonomic to work with and may help type inference.
    pub fn lift<X: Default>(self) -> product_type![Self, X] {
        self.with(Default::default())
    }
}
