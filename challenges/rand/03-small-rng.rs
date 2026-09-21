impl SeedableRng for SmallRng {
    // Fix to 256 bits. Changing this is a breaking change!
    type Seed = [u8; 32];

    #[inline(always)]
    fn from_seed(seed: Self::Seed) -> Self {
        // This is for compatibility with 32-bit platforms where Rng::Seed has a different seed size
        // With MSRV >= 1.77: let seed = *seed.first_chunk().unwrap()
        const LEN: usize = core::mem::size_of::<<Rng as SeedableRng>::Seed>();
        let seed = (&seed[..LEN]).try_into().unwrap();
        SmallRng(Rng::from_seed(seed))
    }

    #[inline(always)]
    fn seed_from_u64(state: u64) -> Self {
        SmallRng(Rng::seed_from_u64(state))
    }
}

impl TryRng for SmallRng {
    type Error = Infallible;

    #[inline(always)]
    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        self.0.try_next_u32()
    }

    #[inline(always)]
    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        self.0.try_next_u64()
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Infallible> {
        self.0.try_fill_bytes(dest)
    }
}
