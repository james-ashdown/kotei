use ::core::cmp;
use ::core::fmt;

use crate::U8F;
use crate::U16F;

/// The 32-bit unsigned fixed-point type.
#[derive(Clone, Copy, Eq, Hash, Ord)]
pub struct U32F<const E: i32>(pub(crate) u32);

impl<const E: i32> U32F<E> {
    /// The smallest value that can be represented by this fixed-point type, equal to 0.
    pub const MIN: Self = Self(u32::MIN);

    /// The largest value that can be represented by this fixed-point type, equal to (2<sup>32</sup> - 1) ⋅ 2<sup>E</sup>.
    pub const MAX: Self = Self(u32::MAX);

    /// The size of this type in bits.
    pub const BITS: u32 = u32::BITS;

    /// Creates a new fixed-point number from an integer significand, equal to `significand` ⋅ 2<sup>E</sup>.
    #[must_use]
    pub const fn new(significand: u32) -> Self {
        Self(significand)
    }

    /// Converts from [`U8F<E>`] losslessly.
    #[must_use]
    pub const fn from_u8f(value: U8F<E>) -> Self {
        Self(value.0 as u32)
    }

    /// Converts from [`U16F<E>`] losslessly.
    #[must_use]
    pub const fn from_u16f(value: U16F<E>) -> Self {
        Self(value.0 as u32)
    }

    /// Raw transutation from [`u32`].
    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Creates a native endian fixed-point number from its memory representation as a byte array in native endian byte order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes), as appropriate, instead.
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_ne_bytes(bytes))
    }

    /// Creates a fixed-point number from its memory representation as a byte array in big endian byte order.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }

    /// Creates a fixed-point number from its memory representation as a byte array in little endian byte order.
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(bytes))
    }

    /// Raw transmutation to [`u32`].
    #[must_use]
    pub const fn to_bits(self) -> u32 {
        self.0
    }

    /// Returns the memory representation of this fixed-point number as a byte array in native byte order.
    #[must_use]
    pub const fn to_ne_bytes(self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in big-endian (network) byte order.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in little-endian byte order.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }

    /// Returns the fixed-point significand, equal to `self` ⋅ 2<sup>-E</sup>.
    #[must_use]
    pub const fn significand(self) -> u32 {
        self.0
    }

    /// Returns the fixed-point exponent.
    #[must_use]
    pub const fn exponent(self) -> i32 {
        E
    }

    /// Computes the base 2 logarithm of `self`, rounded down.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is zero, or if overflow occurred.
    #[must_use]
    #[track_caller]
    pub const fn ilog2(self) -> i32 {
        let x = self.0.ilog2();
        let Some(x) = E.checked_add_unsigned(x) else {
            crate::panic::ilog2();
        };

        x
    }

    /// Computes the base 2 logarithm of `self`, rounded down. Returns `None` if `self` is zero, or if overflow occurred.
    #[must_use]
    pub const fn checked_ilog2(self) -> Option<i32> {
        let Some(x) = self.0.checked_ilog2() else {
            return None;
        };
        let Some(x) = E.checked_add_unsigned(x) else {
            return None;
        };

        Some(x)
    }
}

impl From<U32F<0>> for u32 {
    fn from(value: U32F<0>) -> Self {
        value.0
    }
}

impl From<u32> for U32F<0> {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl<const E: i32> From<U8F<E>> for U32F<E> {
    /// Converts from [`U8F<E>`] losslessly.
    fn from(value: U8F<E>) -> Self {
        Self::from_u8f(value)
    }
}

impl<const E: i32> From<U16F<E>> for U32F<E> {
    /// Converts from [`U16F<E>`] losslessly.
    fn from(value: U16F<E>) -> Self {
        Self::from_u16f(value)
    }
}

impl<const E1: i32, const E2: i32> PartialEq<U32F<E2>> for U32F<E1> {
    fn eq(&self, other: &U32F<E2>) -> bool {
        let mut lhs = self.0;
        let mut rhs = other.0;

        if const { E1 > E2 } && lhs != 0 {
            let shift = const { E1.wrapping_sub(E2).cast_unsigned() };

            if shift > lhs.leading_zeros() {
                return false;
            }

            lhs <<= shift;
        }

        if const { E2 > E1 } && rhs != 0 {
            let shift = const { E2.wrapping_sub(E1).cast_unsigned() };

            if shift > rhs.leading_zeros() {
                return false;
            }

            rhs <<= shift;
        }

        lhs == rhs
    }
}

impl<const E1: i32, const E2: i32> PartialOrd<U32F<E2>> for U32F<E1> {
    fn partial_cmp(&self, other: &U32F<E2>) -> Option<cmp::Ordering> {
        let mut lhs = self.0;
        let mut rhs = other.0;

        if const { E1 > E2 } && lhs != 0 {
            let shift = const { E1.wrapping_sub(E2).cast_unsigned() };

            if shift > lhs.leading_zeros() {
                return Some(cmp::Ordering::Greater);
            }

            lhs <<= shift;
        }

        if const { E2 > E1 } && rhs != 0 {
            let shift = const { E2.wrapping_sub(E1).cast_unsigned() };

            if shift > rhs.leading_zeros() {
                return Some(cmp::Ordering::Less);
            }

            rhs <<= shift;
        }

        PartialOrd::partial_cmp(&lhs, &rhs)
    }
}

impl<const E: i32> fmt::Debug for U32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U32F<{E}")?;

        f.debug_tuple(">").field(&self.0).finish()
    }
}

impl<const E: i32> fmt::Binary for U32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::Octal for U32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::LowerHex for U32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::UpperHex for U32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.to_bits(), f)
    }
}
