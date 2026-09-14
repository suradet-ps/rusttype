/// Create a strategy which selects from the given delegate strategies.
///
/// Each strategy is assigned a non-zero weight which determines how
/// frequently that strategy is chosen. For example, a strategy with a
/// weight of 2 will be chosen twice as frequently as one with a weight of
/// 1\.
///
/// ## Panics
///
/// Panics if `options` is empty or any element has a weight of 0.
///
/// Panics if the sum of the weights overflows a `u32`.
pub fn new_weighted(options: Vec<W<T>>) -> Self {
    assert!(!options.is_empty());
    assert!(
        !options.iter().any(|&(w, _)| 0 == w),
        "Union option has a weight of 0"
    );
    assert!(
        options.iter().map(|&(w, _)| u64::from(w)).sum::<u64>()
            <= u64::from(u32::MAX),
        "Union weights overflow u32"
    );
    let options =
        options.into_iter().map(|(w, v)| (w, Arc::new(v))).collect();
    Self { options }
}

/// Add `other` as an additional alternate strategy with weight 1.
pub fn or(mut self, other: T) -> Self {
    self.options.push((1, Arc::new(other)));
    self
}
