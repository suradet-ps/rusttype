impl Pack {
/// Value is packed in the `width` least-significant bits.
pub(crate) const fn least_significant(width: u32) -> Pack {
    let mask = mask_for(width);

    Pack { mask, shift: 0 }
}

/// Value is packed in the `width` more-significant bits.
pub(crate) const fn then(&self, width: u32) -> Pack {
    let shift = usize::BITS - self.mask.leading_zeros();
    let mask = mask_for(width) << shift;

    Pack { mask, shift }
}

/// Width, in bits, dedicated to storing the value.
pub(crate) const fn width(&self) -> u32 {
    usize::BITS - (self.mask >> self.shift).leading_zeros()
}

/// Max representable value.
pub(crate) const fn max_value(&self) -> usize {
    (1 << self.width()) - 1
}

pub(crate) fn pack(&self, value: usize, base: usize) -> usize {
    assert!(value <= self.max_value());
    (base & !self.mask) | (value << self.shift)
}

pub(crate) fn unpack(&self, src: usize) -> usize {
    unpack(src, self.mask, self.shift)
}
}
