pub fn with_start(&self, start: usize) -> Match {
    assert!(start <= self.end, "{} is not <= {}", start, self.end);
    Match { start, ..*self }
}

/// Return a new match with the end offset replaced with the given
/// value.
///
/// # Panics
///
/// This method panics if `self.start > end`.
#[inline]
pub fn with_end(&self, end: usize) -> Match {
    assert!(self.start <= end, "{} is not <= {}", self.start, end);
    Match { end, ..*self }
}

/// Offset this match by the given amount and return a new match.
///
/// This adds the given offset to the start and end of this match, and
/// returns the resulting match.
///
/// # Panics
///
/// This panics if adding the given amount to either the start or end
/// offset would result in an overflow.
#[inline]
pub fn offset(&self, amount: usize) -> Match {
    Match {
        start: self.start.checked_add(amount).unwrap(),
        end: self.end.checked_add(amount).unwrap(),
    }
}
