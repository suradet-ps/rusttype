/// Checks whether a control tag represents a full bucket (top bit is clear).
#[inline]
pub(crate) const fn is_full(self) -> bool {
    self.0 & 0x80 == 0
}

/// Checks whether a control tag represents a special value (top bit is set).
#[inline]
pub(crate) const fn is_special(self) -> bool {
    self.0 & 0x80 != 0
}

/// Checks whether a special control value is EMPTY (just check 1 bit).
#[inline]
pub(crate) const fn special_is_empty(self) -> bool {
    debug_assert!(self.is_special());
    self.0 & 0x01 != 0
}

/// Creates a control tag representing a full bucket with the given hash.
#[inline]
pub(crate) const fn full(hash: u64) -> Tag {
    // Constant for function that grabs the top 7 bits of the hash.
    const MIN_HASH_LEN: usize = if size_of::<usize>() < size_of::<u64>() {
        size_of::<usize>()
    } else {
        size_of::<u64>()
    };

    // Grab the top 7 bits of the hash. While the hash is normally a full 64-bit
    // value, some hash functions (such as FxHash) produce a usize result
    // instead, which means that the top 32 bits are 0 on 32-bit platforms.
    // So we use MIN_HASH_LEN constant to handle this.
    let top7 = hash >> (MIN_HASH_LEN * 8 - 7);
    Tag((top7 & 0x7f) as u8) // truncation
}
