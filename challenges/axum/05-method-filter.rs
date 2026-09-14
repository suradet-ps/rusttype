    pub const CONNECT: Self = Self::from_bits(0b00_0000_0001);
    /// Match `DELETE` requests.
    pub const DELETE: Self = Self::from_bits(0b00_0000_0010);
    /// Match `GET` requests.
    pub const GET: Self = Self::from_bits(0b00_0000_0100);
    /// Match `HEAD` requests.
    pub const HEAD: Self = Self::from_bits(0b00_0000_1000);
    /// Match `OPTIONS` requests.
    pub const OPTIONS: Self = Self::from_bits(0b00_0001_0000);
    /// Match `PATCH` requests.
    pub const PATCH: Self = Self::from_bits(0b00_0010_0000);
    /// Match `POST` requests.
    pub const POST: Self = Self::from_bits(0b00_0100_0000);
    /// Match `PUT` requests.
    pub const PUT: Self = Self::from_bits(0b00_1000_0000);
    /// Match `TRACE` requests.
    pub const TRACE: Self = Self::from_bits(0b01_0000_0000);
    /// Match `QUERY` requests.
    pub const QUERY: Self = Self::from_bits(0b10_0000_0000);

    const fn bits(self) -> u16 {
        let bits = self;
        bits.0
    }

    const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub(crate) const fn contains(self, other: Self) -> bool {
        self.bits() & other.bits() == other.bits()
    }

    /// Performs the OR operation between the [`MethodFilter`] in `self` with `other`.
    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
