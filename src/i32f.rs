use ::core::fmt;

/// The 32-bit signed fixed-point type.
#[derive(Clone, Copy, Hash)]
pub struct I32F<const E: i32>(pub(crate) i32);

impl<const E: i32> I32F<E> {
    /// The smallest value that can be represented by this fixed-point type, equal to -2<sup>31</sup> ⋅ 2<sup>E</sup>.
    pub const MIN: Self = Self(i32::MIN);

    /// The largest value that can be represented by this fixed-point type, equal to (2<sup>31</sup> - 1) ⋅ 2<sup>E</sup>.
    pub const MAX: Self = Self(i32::MAX);

    /// The size of this type in bits.
    pub const BITS: u32 = i32::BITS;

    /// Creates a new fixed-point number from an integer significand, equal to `significand` ⋅ 2<sup>E</sup>.
    #[must_use]
    pub const fn new(significand: i32) -> Self {
        Self(significand)
    }

    /// Raw transutation from [`u32`].
    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits.cast_signed())
    }

    /// Creates a native endian fixed-point number from its memory representation as a byte array in native endian byte order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes), as appropriate, instead.
    #[inline(always)]
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; 4]) -> Self {
        Self(i32::from_ne_bytes(bytes))
    }

    /// Creates a fixed-point number from its memory representation as a byte array in big endian byte order.
    #[inline(always)]
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(i32::from_be_bytes(bytes))
    }

    /// Creates a fixed-point number from its memory representation as a byte array in little endian byte order.
    #[inline(always)]
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self(i32::from_le_bytes(bytes))
    }

    /// Raw transmutation to [`u32`].
    #[inline(always)]
    #[must_use]
    pub const fn to_bits(self) -> u32 {
        self.0.cast_unsigned()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in native byte order.
    #[inline(always)]
    #[must_use]
    pub const fn to_ne_bytes(self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in big-endian (network) byte order.
    #[inline(always)]
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in little-endian byte order.
    #[inline(always)]
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }

    /// Returns the fixed-point significand, equal to `self` ⋅ 2<sup>-E</sup>.
    #[must_use]
    pub const fn significand(self) -> i32 {
        self.0
    }

    /// Returns the fixed-point exponent.
    #[must_use]
    pub const fn exponent(self) -> i32 {
        E
    }
}

impl<const E: i32> fmt::Debug for I32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I32F<{E}")?;
        f.debug_tuple(">").field(&self.0).finish()
    }
}

impl<const E: i32> fmt::Binary for I32F<E> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::LowerHex for I32F<E> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::Octal for I32F<E> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::UpperHex for I32F<E> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.to_bits(), f)
    }
}
