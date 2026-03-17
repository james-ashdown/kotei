use ::core::cmp;
use ::core::fmt;
use ::core::ops;

/// The 32-bit signed fixed-point type.
#[derive(Clone, Copy, Eq, Hash, Ord)]
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
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; 4]) -> Self {
        Self(i32::from_ne_bytes(bytes))
    }

    /// Creates a fixed-point number from its memory representation as a byte array in big endian byte order.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(i32::from_be_bytes(bytes))
    }

    /// Creates a fixed-point number from its memory representation as a byte array in little endian byte order.
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self(i32::from_le_bytes(bytes))
    }

    /// Returns the nearest [`f32`] to `self`, rounded to the number with even least significant digits if `self` is halfway between two representable [`f32`] numbers, saturating at [`f32::INFINITY`] or [`f32::NEG_INFINITY`] if `self` rounds to a value greater than [`f32::MAX`] or less than [`f32::MIN`], respectively.
    #[must_use]
    pub const fn to_f32(self) -> f32 {
        if E >= -126 {
            let scaling_factor = const {
                let mut exponent = 127u32.saturating_add_signed(E);

                if exponent > 0x000000FF {
                    exponent = 0x000000FF;
                }

                let bits = exponent << 23;

                f32::from_bits(bits)
            };

            if scaling_factor == f32::INFINITY && self.0 == 0 {
                0.0
            } else {
                self.0 as f32 * scaling_factor
            }
        } else {
            let mut bits = 0;
            let mut significand = self.0.cast_unsigned();

            if self.0 < 0 {
                bits |= 0x80000000;
                significand = significand.wrapping_neg();
            }

            let leading_zeros = significand.leading_zeros();
            let mut exponent = const { 127 + 31 } - leading_zeros;
            let align =
                8 + leading_zeros.saturating_sub_signed(const { E.saturating_add(126 + 31) });

            if leading_zeros >= align {
                let shift = leading_zeros - align;
                significand <<= shift;
            } else {
                let shift = align - leading_zeros;

                if shift >= u32::BITS {
                    significand = 0;
                } else {
                    significand += significand >> shift & 0x00000001;
                    significand += !(!0 << (shift - 1));
                    significand >>= shift;

                    if significand.leading_zeros() < 8 {
                        significand >>= 1;
                        exponent += 1;
                    }
                }
            }

            exponent = exponent.saturating_add_signed(E);
            bits |= exponent << 23;
            bits |= significand & 0x007FFFFF;

            f32::from_bits(bits)
        }
    }

    /// Raw transmutation to [`u32`].
    #[must_use]
    pub const fn to_bits(self) -> u32 {
        self.0.cast_unsigned()
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
    pub const fn significand(self) -> i32 {
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
    /// This function will panic if `self` is zero or negative, or if overflow occurred.
    #[must_use]
    #[track_caller]
    pub const fn ilog2(self) -> i32 {
        let x = self.0.ilog2();
        let Some(x) = E.checked_add_unsigned(x) else {
            crate::panic::ilog2();
        };

        x
    }

    /// Computes the base 2 logarithm of `self`, rounded down. Returns `None` if `self` is zero or negative, or if overflow occurred.
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
}

impl From<I32F<0>> for i32 {
    fn from(value: I32F<0>) -> Self {
        value.0
    }
}

impl From<i32> for I32F<0> {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl<const E1: i32, const E2: i32> PartialEq<I32F<E2>> for I32F<E1> {
    fn eq(&self, other: &I32F<E2>) -> bool {
        let mut lhs = self.0;
        let mut rhs = other.0;

        if const { E1 > E2 } && lhs != 0 {
            if const { E1.abs_diff(E2) } >= lhs.leading_zeros() | lhs.leading_ones() {
                return false;
            }

            lhs <<= const { E1.abs_diff(E2) };
        }

        if const { E2 > E1 } && rhs != 0 {
            if const { E2.abs_diff(E1) } >= rhs.leading_zeros() | rhs.leading_ones() {
                return false;
            }

            rhs <<= const { E2.abs_diff(E1) };
        }

        lhs == rhs
    }
}

impl<const E1: i32, const E2: i32> PartialOrd<I32F<E2>> for I32F<E1> {
    fn partial_cmp(&self, other: &I32F<E2>) -> Option<cmp::Ordering> {
        let mut lhs = self.0;
        let mut rhs = other.0;

        if const { E1 > E2 } && lhs != 0 {
            if const { E1.abs_diff(E2) } >= lhs.leading_zeros() | lhs.leading_ones() {
                if lhs > 0 {
                    return Some(cmp::Ordering::Greater);
                } else {
                    return Some(cmp::Ordering::Less);
                }
            }

            lhs <<= const { E1.abs_diff(E2) };
        }

        if const { E2 > E1 } && rhs != 0 {
            if const { E2.abs_diff(E1) } >= rhs.leading_zeros() | rhs.leading_ones() {
                if rhs > 0 {
                    return Some(cmp::Ordering::Less);
                } else {
                    return Some(cmp::Ordering::Greater);
                }
            }

            rhs <<= const { E2.abs_diff(E1) };
        }

        PartialOrd::partial_cmp(&lhs, &rhs)
    }
}

impl<const E: i32> fmt::Debug for I32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I32F<{E}")?;
        f.debug_tuple(">").field(&self.0).finish()
    }
}

impl<const E: i32> fmt::Binary for I32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::Octal for I32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::LowerHex for I32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::UpperHex for I32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> ops::Neg for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<const E: i32> ops::Add for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<const E: i32> ops::Sub for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp_greater_than_max() {
        assert!(I32F::<0>::new(1) > I32F::<-31>::MAX);
        assert!(I32F::<-31>::MAX < I32F::<0>::new(1));
    }

    #[test]
    fn eq_one() {
        assert_eq!(I32F::<0>::new(1), I32F::<0>::new(1));
        assert_eq!(I32F::<0>::new(1), I32F::<-1>::new(2));
        assert_eq!(I32F::<-1>::new(2), I32F::<0>::new(1));
    }

    #[test]
    fn eq_zero() {
        assert_eq!(I32F::<0>::new(0), I32F::<0>::new(0));
        assert_eq!(I32F::<0>::new(0), I32F::<1>::new(0));
        assert_eq!(I32F::<1>::new(0), I32F::<0>::new(0));
    }

    #[test]
    fn to_f32_half() {
        assert_eq!(I32F::<-1>::new(1).to_f32(), 0.5);
    }

    #[test]
    fn to_f32_max() {
        assert!(I32F::<{ f32::MAX_EXP }>::new(1).to_f32() > f32::MAX);
        assert_eq!(
            I32F::<{ f32::MAX_EXP - 24 }>::from_bits(0x00FFFFFF).to_f32(),
            f32::MAX
        );
    }

    #[test]
    fn to_f32_max_negative() {
        assert_eq!(
            I32F::<{ f32::MIN_EXP - 1 }>::new(-1).to_f32(),
            -f32::MIN_POSITIVE
        );
    }

    #[test]
    fn to_f32_max_negative_subnormal() {
        assert_eq!(I32F::<-149>::new(-1).to_f32(), (-0.0f32).next_down());
    }

    #[test]
    fn to_f32_min() {
        assert!(I32F::<{ f32::MAX_EXP }>::new(-1).to_f32() < f32::MIN);
        assert_eq!(
            I32F::<{ f32::MAX_EXP - 24 }>::from_bits(0xFF000001).to_f32(),
            f32::MIN
        );
    }

    #[test]
    fn to_f32_min_positive() {
        assert_eq!(
            I32F::<{ f32::MIN_EXP - 1 }>::new(1).to_f32(),
            f32::MIN_POSITIVE
        );
    }

    #[test]
    fn to_f32_min_positive_subnormal() {
        assert_eq!(I32F::<-149>::new(1).to_f32(), 0.0f32.next_up());
    }

    #[test]
    fn to_f32_one() {
        assert_eq!(I32F::<0>::new(1).to_f32(), 1.0);
    }

    #[test]
    fn to_f32_round_once() {
        let x = 0x01000005;
        assert_eq!(I32F::<-152>::from_bits(x).to_f32().to_bits(), 0x00200001);
        assert_eq!(
            (x as f32 * 2.0f32.powi(-76) * 2.0f32.powi(-76)).to_bits(),
            0x00200000
        );
    }

    #[test]
    fn to_f32_two() {
        assert_eq!(I32F::<1>::new(1).to_f32(), 2.0);
    }

    #[test]
    fn to_f32_zero() {
        assert_eq!(I32F::<{ i32::MIN }>::new(0).to_f32(), 0.0);
        assert_eq!(I32F::<0>::new(0).to_f32(), 0.0);
        assert_eq!(I32F::<{ i32::MAX }>::new(0).to_f32(), 0.0);
    }
}
