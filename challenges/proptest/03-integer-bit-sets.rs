macro_rules! int_bitset {
    ($typ:ty) => {
        impl BitSetLike for $typ {
            fn new_bitset(_: usize) -> Self {
                0
            }
            fn len(&self) -> usize {
                mem::size_of::<$typ>() * 8
            }
            fn test(&self, ix: usize) -> bool {
                0 != (*self & ((1 as $typ) << ix))
            }
            fn set(&mut self, ix: usize) {
                *self |= (1 as $typ) << ix;
            }
            fn clear(&mut self, ix: usize) {
                *self &= !((1 as $typ) << ix);
            }
            fn count(&self) -> usize {
                self.count_ones() as usize
            }
        }
    };
}
int_bitset!(u8);
int_bitset!(u16);
int_bitset!(u32);
int_bitset!(u64);
int_bitset!(u128);
int_bitset!(usize);
int_bitset!(i8);
int_bitset!(i16);
int_bitset!(i32);
int_bitset!(i64);
int_bitset!(i128);
int_bitset!(isize);
