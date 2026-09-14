/// Trait for types which can be handled with `BitSetStrategy`.
#[cfg_attr(clippy, allow(len_without_is_empty))]
pub trait BitSetLike: Clone + fmt::Debug {
    /// Create a new value of `Self` with space for up to `max` bits, all
    /// initialised to zero.
    fn new_bitset(max: usize) -> Self;
    /// Return an upper bound on the greatest bit set _plus one_.
    fn len(&self) -> usize;
    /// Test whether the given bit is set.
    fn test(&self, ix: usize) -> bool;
    /// Set the given bit.
    fn set(&mut self, ix: usize);
    /// Clear the given bit.
    fn clear(&mut self, ix: usize);
    /// Return the number of bits set.
    ///
    /// This has a default for backwards compatibility, which simply does a
    /// linear scan through the bits. Implementations are strongly encouraged
    /// to override this.
    fn count(&self) -> usize {
        let mut n = 0;
        for i in 0..self.len() {
            if self.test(i) {
                n += 1;
            }
        }
        n
    }
}
