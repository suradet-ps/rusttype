pub(crate) struct CoinFlipper<R: Rng> {
    pub rng: R,
    chunk: u32, // TODO(opt): this should depend on RNG word size
    chunk_remaining: u32,
}

impl<R: Rng> CoinFlipper<R> {
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            chunk: 0,
            chunk_remaining: 0,
        }
    }

    #[inline]
    /// Returns true with a probability of 1 / d
    /// Uses an expected two bits of randomness
    /// Panics if d == 0
    pub fn random_ratio_one_over(&mut self, d: usize) -> bool {
        debug_assert_ne!(d, 0);
        // This uses the same logic as `random_ratio` but is optimized for the case that
        // the starting numerator is one (which it always is for `Sequence::Choose()`)

        // In this case (but not `random_ratio`), this way of calculating c is always accurate
        let c = (usize::BITS - 1 - d.leading_zeros()).min(32);

        if self.flip_c_heads(c) {
            let numerator = 1 << c;
            self.random_ratio(numerator, d)
        } else {
            false
        }
    }
