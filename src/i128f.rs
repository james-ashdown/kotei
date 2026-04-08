use ::core::cmp;
use ::core::fmt;
use ::core::ops;

use crate::I8F;
use crate::I16F;
use crate::I32F;
use crate::I64F;
use crate::U8F;
use crate::U16F;
use crate::U32F;
use crate::U64F;
use crate::U128F;

/// The 32-bit unsigned fixed-point type.
#[derive(Clone, Copy, Eq, Hash, Ord)]
pub struct I128F<const E: i32>(pub(crate) i128);

impl<const E: i32> I128F<E> {
    /// The smallest value that can be represented by this fixed-point type, equal to -2<sup>127</sup> ⋅ 2<sup>E</sup>.
    pub const MIN: Self = Self(i128::MIN);

    /// The largest value that can be represented by this fixed-point type, equal to (2<sup>127</sup> - 1) ⋅ 2<sup>E</sup>.
    pub const MAX: Self = Self(i128::MAX);

    /// The size of this type in bits.
    pub const BITS: u32 = i128::BITS;

    /// Creates a new fixed-point number from an integer significand, equal to `significand` ⋅ 2<sup>E</sup>.
    #[must_use]
    pub const fn new(significand: i128) -> Self {
        Self(significand)
    }

    /// Converts from [`I8F<E>`] losslessly.
    #[must_use]
    pub const fn from_i8f(value: I8F<E>) -> Self {
        Self(value.0 as i128)
    }

    /// Converts from [`I16F<E>`] losslessly.
    #[must_use]
    pub const fn from_i16f(value: I16F<E>) -> Self {
        Self(value.0 as i128)
    }

    /// Converts from [`I32F<E>`] losslessly.
    #[must_use]
    pub const fn from_i32f(value: I32F<E>) -> Self {
        Self(value.0 as i128)
    }

    /// Converts from [`I64F<E>`] losslessly.
    #[must_use]
    pub const fn from_i64f(value: I64F<E>) -> Self {
        Self(value.0 as i128)
    }

    /// Converts from [`U8F<E>`] losslessly.
    #[must_use]
    pub const fn from_u8f(value: U8F<E>) -> Self {
        Self(value.0 as i128)
    }

    /// Converts from [`U16F<E>`] losslessly.
    #[must_use]
    pub const fn from_u16f(value: U16F<E>) -> Self {
        Self(value.0 as i128)
    }

    /// Converts from [`U32F<E>`] losslessly.
    #[must_use]
    pub const fn from_u32f(value: U32F<E>) -> Self {
        Self(value.0 as i128)
    }

    /// Converts from [`U64F<E>`] losslessly.
    #[must_use]
    pub const fn from_u64f(value: U64F<E>) -> Self {
        Self(value.0 as i128)
    }

    /// Raw transutation from [`u128`].
    #[must_use]
    pub const fn from_bits(bits: u128) -> Self {
        Self(bits.cast_signed())
    }

    /// Creates a native endian fixed-point number from its memory representation as a byte array in native endian byte order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes), as appropriate, instead.
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; 16]) -> Self {
        Self(i128::from_ne_bytes(bytes))
    }

    /// Creates a fixed-point number from its memory representation as a byte array in big endian byte order.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 16]) -> Self {
        Self(i128::from_be_bytes(bytes))
    }

    /// Creates a fixed-point number from its memory representation as a byte array in little endian byte order.
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; 16]) -> Self {
        Self(i128::from_le_bytes(bytes))
    }

    /// Raw transmutation to [`u128`].
    #[must_use]
    pub const fn to_bits(self) -> u128 {
        self.0.cast_unsigned()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in native byte order.
    #[must_use]
    pub const fn to_ne_bytes(self) -> [u8; 16] {
        self.0.to_ne_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in big-endian (network) byte order.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in little-endian byte order.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 16] {
        self.0.to_le_bytes()
    }

    /// Returns the fixed-point significand, equal to `self` ⋅ 2<sup>-E</sup>.
    #[must_use]
    pub const fn significand(self) -> i128 {
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

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn neg(self) -> Self {
        Self(-self.0)
    }

    /// Computes `-self`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_neg(self) -> Self {
        Self(self.0.strict_neg())
    }

    /// Computes `-self`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_neg(self) -> Self {
        Self(self.0.wrapping_neg())
    }

    /// Computes `-self`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_neg(self) -> Self {
        Self(self.0.saturating_neg())
    }

    /// Computes `-self`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        let (x, overflow) = self.0.overflowing_neg();

        (Self(x), overflow)
    }

    /// Computes `-self`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_neg(self) -> Option<Self> {
        let Some(x) = self.0.checked_neg() else {
            return None;
        };

        Some(Self(x))
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }

    /// Computes `self + rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_add(self, rhs: Self) -> Self {
        Self(self.0.strict_add(rhs.0))
    }

    /// Computes `self + rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }

    /// Computes `self + rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }

    /// Computes `self + rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (x, overflowed) = self.0.overflowing_add(rhs.0);

        (Self(x), overflowed)
    }

    /// Computes `self + rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        let Some(x) = self.0.checked_add(rhs.0) else {
            return None;
        };

        Some(Self(x))
    }

    /// Computes `self + rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn add_unsigned(self, rhs: U128F<E>) -> Self {
        let x = self.0.wrapping_add_unsigned(rhs.0);

        if cfg!(debug_assertions) && x < self.0 {
            crate::panic::add();
        }

        Self(x)
    }

    /// Computes `self + rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_add_unsigned(self, rhs: U128F<E>) -> Self {
        let x = self.0.wrapping_add_unsigned(rhs.0);

        if x < self.0 {
            crate::panic::add();
        }

        Self(x)
    }

    /// Computes `self + rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_add_unsigned(self, rhs: U128F<E>) -> Self {
        Self(self.0.wrapping_add_unsigned(rhs.0))
    }

    /// Computes `self + rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_add_unsigned(self, rhs: U128F<E>) -> Self {
        let x = self.0.wrapping_add_unsigned(rhs.0);

        if x < self.0 {
            return Self::MAX;
        }

        Self(x)
    }

    /// Computes `self + rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_add_unsigned(self, rhs: U128F<E>) -> (Self, bool) {
        let x = self.0.wrapping_add_unsigned(rhs.0);

        (Self(x), x < self.0)
    }

    /// Computes `self + rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_add_unsigned(self, rhs: U128F<E>) -> Option<Self> {
        let x = self.0.wrapping_add_unsigned(rhs.0);

        if x < self.0 {
            return None;
        }

        Some(Self(x))
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }

    /// Computes `self - rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_sub(self, rhs: Self) -> Self {
        Self(self.0.strict_sub(rhs.0))
    }

    /// Computes `self - rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    /// Computes `self - rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    /// Computes `self - rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (x, overflowed) = self.0.overflowing_sub(rhs.0);

        (Self(x), overflowed)
    }

    /// Computes `self - rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        let Some(x) = self.0.checked_sub(rhs.0) else {
            return None;
        };

        Some(Self(x))
    }

    /// Computes `self - rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn sub_unsigned(self, rhs: U128F<E>) -> Self {
        let x = self.0.wrapping_sub_unsigned(rhs.0);

        if cfg!(debug_assertions) && x > self.0 {
            crate::panic::sub();
        }

        Self(x)
    }

    /// Computes `self - rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_sub_unsigned(self, rhs: U128F<E>) -> Self {
        let x = self.0.wrapping_sub_unsigned(rhs.0);

        if x > self.0 {
            crate::panic::sub();
        }

        Self(x)
    }

    /// Computes `self - rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_sub_unsigned(self, rhs: U128F<E>) -> Self {
        Self(self.0.wrapping_sub_unsigned(rhs.0))
    }

    /// Computes `self - rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_sub_unsigned(self, rhs: U128F<E>) -> Self {
        let x = self.0.wrapping_sub_unsigned(rhs.0);

        if x > self.0 {
            return Self::MIN;
        }

        Self(x)
    }

    /// Computes `self - rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_sub_unsigned(self, rhs: U128F<E>) -> (Self, bool) {
        let x = self.0.wrapping_sub_unsigned(rhs.0);

        (Self(x), x > self.0)
    }

    /// Computes `self - rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_sub_unsigned(self, rhs: U128F<E>) -> Option<Self> {
        let x = self.0.wrapping_sub_unsigned(rhs.0);

        if x > self.0 {
            return None;
        }

        Some(Self(x))
    }
}

impl From<I128F<0>> for i128 {
    fn from(value: I128F<0>) -> Self {
        value.0
    }
}

impl From<i128> for I128F<0> {
    fn from(value: i128) -> Self {
        Self(value)
    }
}

impl<const E: i32> From<I8F<E>> for I128F<E> {
    fn from(value: I8F<E>) -> Self {
        Self::from_i8f(value)
    }
}

impl<const E: i32> From<I16F<E>> for I128F<E> {
    fn from(value: I16F<E>) -> Self {
        Self::from_i16f(value)
    }
}

impl<const E: i32> From<I32F<E>> for I128F<E> {
    fn from(value: I32F<E>) -> Self {
        Self::from_i32f(value)
    }
}

impl<const E: i32> From<I64F<E>> for I128F<E> {
    fn from(value: I64F<E>) -> Self {
        Self::from_i64f(value)
    }
}

impl<const E: i32> From<U8F<E>> for I128F<E> {
    fn from(value: U8F<E>) -> Self {
        Self::from_u8f(value)
    }
}

impl<const E: i32> From<U16F<E>> for I128F<E> {
    fn from(value: U16F<E>) -> Self {
        Self::from_u16f(value)
    }
}

impl<const E: i32> From<U32F<E>> for I128F<E> {
    fn from(value: U32F<E>) -> Self {
        Self::from_u32f(value)
    }
}

impl<const E: i32> From<U64F<E>> for I128F<E> {
    fn from(value: U64F<E>) -> Self {
        Self::from_u64f(value)
    }
}

impl<const E1: i32, const E2: i32> PartialEq<I128F<E2>> for I128F<E1> {
    fn eq(&self, other: &I128F<E2>) -> bool {
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

impl<const E1: i32, const E2: i32> PartialOrd<I128F<E2>> for I128F<E1> {
    fn partial_cmp(&self, other: &I128F<E2>) -> Option<cmp::Ordering> {
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

impl<const E: i32> fmt::Debug for I128F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I128F<{E}")?;

        f.debug_tuple(">").field(&self.0).finish()
    }
}

impl<const E: i32> fmt::Binary for I128F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::Octal for I128F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::LowerHex for I128F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::UpperHex for I128F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> ops::Neg for I128F<E> {
    type Output = Self;

    #[track_caller]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<const E: i32> ops::Add for I128F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<const E: i32> ops::Sub for I128F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
