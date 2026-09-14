pub(crate) fn match_tag(self, tag: Tag) -> BitMask {
    // This algorithm is derived from
    // https://graphics.stanford.edu/~seander/bithacks.html##ValueInWord
    let cmp = self.0 ^ repeat(tag);
    BitMask((cmp.wrapping_sub(repeat(Tag(0x01))) & !cmp & repeat(Tag::DELETED)).to_le())
}

/// Returns a `BitMask` indicating all tags in the group which are
/// `EMPTY`.
#[inline]
pub(crate) fn match_empty(self) -> BitMask {
    // If the high bit is set, then the tag must be either:
    // 1111_1111 (EMPTY) or 1000_0000 (DELETED).
    // So we can just check if the top two bits are 1 by ANDing them.
    BitMask((self.0 & (self.0 << 1) & repeat(Tag::DELETED)).to_le())
}

/// Returns a `BitMask` indicating all tags in the group which are
/// `EMPTY` or `DELETED`.
#[inline]
pub(crate) fn match_empty_or_deleted(self) -> BitMask {
    // A tag is EMPTY or DELETED iff the high bit is set
    BitMask((self.0 & repeat(Tag::DELETED)).to_le())
}

/// Returns a `BitMask` indicating all tags in the group which are full.
#[inline]
pub(crate) fn match_full(self) -> BitMask {
    BitMask(self.match_empty_or_deleted().0 ^ BITMASK_MASK)
}
