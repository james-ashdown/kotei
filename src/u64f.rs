use ::core::cmp;
use ::core::fmt;
use ::core::ops;

use crate::I64F;
use crate::U8F;
use crate::U16F;
use crate::U32F;
use crate::error::TryFromFloatError;

/// The 32-bit unsigned fixed-point type.
#[derive(Clone, Copy, Eq, Hash, Ord)]
pub struct U64F<const E: i32>(pub(crate) u64);

impl U64F<-66> {
    /// 1/τ
    pub const FRAC_1_TAU: Self = Self::from_bits(0xA2F9836E4E44152A);
}

impl U64F<-65> {
    /// 1/π
    pub const FRAC_1_PI: Self = Self::from_bits(0xA2F9836E4E44152A);
    /// π/8
    pub const FRAC_PI_8: Self = Self::from_bits(0xC90FDAA22168C235);
    /// log<sub>10</sub>(2)
    pub const LOG10_2: Self = Self::from_bits(0x9A209A84FBCFF799);
    /// log<sub>10</sub>(e)
    pub const LOG10_E: Self = Self::from_bits(0xDE5BD8A937287195);
}

impl U64F<-64> {
    /// The Euler-Mascheroni constant (γ)
    pub const EULER_GAMMA: Self = Self::from_bits(0x93C467E37DB0C7A5);
    /// 1/sqrt(2)
    pub const FRAC_1_SQRT_2: Self = Self::from_bits(0xB504F333F9DE6484);
    /// 2/π
    pub const FRAC_2_PI: Self = Self::from_bits(0xA2F9836E4E44152A);
    /// π/4
    pub const FRAC_PI_4: Self = Self::from_bits(0xC90FDAA22168C235);
    /// π/6
    pub const FRAC_PI_6: Self = Self::from_bits(0x860A91C16B9B2C23);
    /// ln(2)
    pub const LN_2: Self = Self::from_bits(0xB17217F7D1CF79AC);
}

impl U64F<-63> {
    /// 2/sqrt(π)
    pub const FRAC_2_SQRT_PI: Self = Self::from_bits(0x906EBA8214DB688D);
    /// π/2
    pub const FRAC_PI_2: Self = Self::from_bits(0xC90FDAA22168C235);
    /// π/3
    pub const FRAC_PI_3: Self = Self::from_bits(0x860A91C16B9B2C23);
    /// The golden ratio (φ)
    pub const GOLDEN_RATIO: Self = Self::from_bits(0xCF1BBCDCBFA53E0B);
    /// log<sub>2</sub>(e)
    pub const LOG2_E: Self = Self::from_bits(0xB8AA3B295C17F0BC);
    /// sqrt(2)
    pub const SQRT_2: Self = Self::from_bits(0xB504F333F9DE6484);
}

impl U64F<-62> {
    /// Euler's number (e)
    pub const E: Self = Self::from_bits(0xADF85458A2BB4A9B);
    /// ln(10)
    pub const LN_10: Self = Self::from_bits(0x935D8DDDAAA8AC17);
    /// log<sub>2</sub>(10)
    pub const LOG2_10: Self = Self::from_bits(0xD49A784BCD1B8AFE);
    /// Archimedes’ constant (π)
    pub const PI: Self = Self::from_bits(0xC90FDAA22168C235);
}

impl U64F<-61> {
    /// The full circle constant (τ)
    pub const TAU: Self = Self::from_bits(0xC90FDAA22168C235);
}

impl<const E: i32> U64F<E> {
    /// The smallest value that can be represented by this fixed-point type, equal to 0.
    pub const MIN: Self = Self(u64::MIN);

    /// The largest value that can be represented by this fixed-point type, equal to (2<sup>64</sup> - 1) ⋅ 2<sup>E</sup>.
    pub const MAX: Self = Self(u64::MAX);

    /// The size of this type in bits.
    pub const BITS: u32 = u64::BITS;

    /// Creates a new fixed-point number from an integer significand, equal to `significand` ⋅ 2<sup>E</sup>.
    #[must_use]
    pub const fn new(significand: u64) -> Self {
        Self(significand)
    }

    /// Tries to create a new fixed-point number from [`f32`]. Returns the nearest multiple of 2<sup>E</sup> to `value`, rounded to the number with even least significant digits if `value` is halfway between two multiples of 2<sup>E</sup>. Returns an error if `value` is not a number, less than [`Self::MIN`], or greater than [`Self::MAX`].
    pub const fn try_new_from_f32(value: f32) -> Result<Self, TryFromFloatError> {
        let bits = value.to_bits();

        if bits & 0x7FFFFFFF == 0 {
            return Ok(Self(0));
        }

        let mut significand = bits & 0x7FFFFF;
        let mut exponent = bits >> 23 & 0xFF;
        let negative = bits >> 31 != 0;

        if exponent == 0xFF {
            if significand != 0 {
                return Err(TryFromFloatError::Nan);
            } else if negative {
                return Err(TryFromFloatError::Underflow);
            } else {
                return Err(TryFromFloatError::Overflow);
            }
        } else if exponent > 0 {
            significand |= 0x800000;
        } else {
            exponent = 1;
        }

        let mut significand = significand as u64;
        let exponent = exponent as i32 - const { 127 + 23 };

        if exponent >= E {
            let shift = exponent.wrapping_sub(E) as u32;

            if shift >= significand.leading_zeros() {
                if negative {
                    return Err(TryFromFloatError::Underflow);
                } else {
                    return Err(TryFromFloatError::Overflow);
                }
            } else {
                significand <<= shift;
            }
        } else {
            let shift = E.wrapping_sub(exponent) as u32;

            if shift >= u64::BITS {
                significand = 0;
            } else {
                significand = significand.wrapping_add(significand >> shift & 0x1);
                significand = significand.wrapping_add(!(!0 << shift.wrapping_sub(1)));
                significand >>= shift;
            }
        }

        if negative && significand > 0 {
            return Err(TryFromFloatError::Underflow);
        }

        Ok(Self(significand))
    }

    /// Tries to create a new fixed-point number from [`f64`]. Returns the nearest multiple of 2<sup>E</sup> to `value`, rounded to the number with even least significant digits if `value` is halfway between two multiples of 2<sup>E</sup>. Returns an error if `value` is not a number, less than [`Self::MIN`], or greater than [`Self::MAX`].
    pub const fn try_new_from_f64(value: f64) -> Result<Self, TryFromFloatError> {
        let bits = value.to_bits();

        if bits & 0x7FFFFFFFFFFFFFFF == 0 {
            return Ok(Self(0));
        }

        let mut significand = bits & 0xFFFFFFFFFFFFF;
        let mut exponent = bits >> 52 & 0x7FF;
        let negative = bits >> 63 != 0;

        if exponent == 0x7FF {
            if significand != 0 {
                return Err(TryFromFloatError::Nan);
            } else if negative {
                return Err(TryFromFloatError::Underflow);
            } else {
                return Err(TryFromFloatError::Overflow);
            }
        } else if exponent > 0 {
            significand |= 0x10000000000000;
        } else {
            exponent = 1;
        }

        let exponent = (exponent as i32).wrapping_sub(const { 1023 + 52 });

        if exponent >= E {
            let shift = exponent.wrapping_sub(E) as u32;

            if shift >= significand.leading_zeros() {
                if negative {
                    return Err(TryFromFloatError::Underflow);
                } else {
                    return Err(TryFromFloatError::Overflow);
                }
            } else {
                significand <<= shift;
            }
        } else {
            let shift = E.wrapping_sub(exponent) as u32;

            if shift >= u64::BITS {
                significand = 0;
            } else {
                significand = significand.wrapping_add(significand >> shift & 0x1);
                significand = significand.wrapping_add(!(!0 << shift.wrapping_sub(1)));
                significand >>= shift;
            }
        }

        if negative && significand > 0 {
            return Err(TryFromFloatError::Underflow);
        }

        Ok(Self(significand))
    }

    /// Converts from [`U8F<E>`] losslessly.
    #[must_use]
    pub const fn from_u8f(value: U8F<E>) -> Self {
        Self(value.0 as u64)
    }

    /// Converts from [`U16F<E>`] losslessly.
    #[must_use]
    pub const fn from_u16f(value: U16F<E>) -> Self {
        Self(value.0 as u64)
    }

    /// Converts from [`U32F<E>`] losslessly.
    #[must_use]
    pub const fn from_u32f(value: U32F<E>) -> Self {
        Self(value.0 as u64)
    }

    /// Raw transutation from [`u64`].
    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Creates a native endian fixed-point number from its memory representation as a byte array in native endian byte order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes), as appropriate, instead.
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_ne_bytes(bytes))
    }

    /// Creates a fixed-point number from its memory representation as a byte array in big endian byte order.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_be_bytes(bytes))
    }

    /// Creates a fixed-point number from its memory representation as a byte array in little endian byte order.
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(bytes))
    }

    /// Returns the nearest [`f32`] to `self`, rounded to the number with even least significant digits if `self` is halfway between two representable [`f32`] numbers, saturating at [`f32::INFINITY`] or [`f32::NEG_INFINITY`] if `self` rounds to a value greater than [`f32::MAX`] or less than [`f32::MIN`], respectively.
    #[must_use]
    pub const fn to_f32(self) -> f32 {
        const BIAS: u32 = 127;

        if const { E >= f32::MIN_EXP - 1 } {
            let scaling_factor = const {
                let mut exponent = 127u32.saturating_add_signed(E);

                if exponent > 0xFF {
                    exponent = 0xFF;
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
            let mut significand = self.0;

            let leading_zeros = significand.leading_zeros();
            let mut exponent = const { BIAS + u64::BITS - 1 }.wrapping_sub(leading_zeros);
            let mut align = const { u64::BITS - f32::MANTISSA_DIGITS };
            align = align.wrapping_add(leading_zeros.saturating_sub_signed(
                const { E.saturating_add_unsigned((BIAS - 1) + (u64::BITS - 1)) },
            ));

            if leading_zeros >= align {
                let shift = leading_zeros.wrapping_sub(align);
                significand <<= shift;
            } else {
                let shift = align.wrapping_sub(leading_zeros);

                if shift >= u64::BITS {
                    significand = 0;
                } else {
                    significand = significand.wrapping_add(significand >> shift & 0x1);
                    significand = significand.wrapping_add(!(!0 << shift.wrapping_sub(1)));
                    significand >>= shift;

                    if significand.leading_zeros() < 8 {
                        significand >>= 1;
                        exponent = exponent.wrapping_add(1);
                    }
                }
            }

            exponent = exponent.saturating_add_signed(E);
            bits |= exponent << 23;
            bits |= (significand as u32) & 0x7FFFFF;

            f32::from_bits(bits)
        }
    }

    /// Returns the nearest [`f32`] to `self`, rounded to the number with even least significant digits if `self` is halfway between two representable [`f32`] numbers, saturating at [`f32::INFINITY`] or [`f32::NEG_INFINITY`] if `self` rounds to a value greater than [`f32::MAX`] or less than [`f32::MIN`], respectively.
    #[must_use]
    pub const fn to_f64(self) -> f64 {
        const BIAS: u32 = 1023;

        if const { E >= f64::MIN_EXP - 1 } {
            let scaling_factor = const {
                let mut exponent = 1023u64.saturating_add_signed(E as i64);

                if exponent > 0x7FF {
                    exponent = 0x7FF;
                }

                let bits = exponent << 52;

                f64::from_bits(bits)
            };

            if scaling_factor == f64::INFINITY && self.0 == 0 {
                0.0
            } else {
                self.0 as f64 * scaling_factor
            }
        } else {
            let mut bits = 0;
            let mut significand = self.0;

            let leading_zeros = significand.leading_zeros();
            let mut exponent = const { BIAS + u64::BITS - 1 }.wrapping_sub(leading_zeros);
            let mut align = const { u64::BITS - f64::MANTISSA_DIGITS };
            align = align.wrapping_add(leading_zeros.saturating_sub_signed(
                const { E.saturating_add_unsigned((BIAS - 1) + (u64::BITS - 1)) },
            ));

            if leading_zeros >= align {
                let shift = leading_zeros.wrapping_sub(align);
                significand <<= shift;
            } else {
                let shift = align.wrapping_sub(leading_zeros);

                if shift >= u64::BITS {
                    significand = 0;
                } else {
                    significand = significand.wrapping_add(significand >> shift & 0x1);
                    significand = significand.wrapping_add(!(!0 << shift.wrapping_sub(1)));
                    significand >>= shift;

                    if significand.leading_zeros() < 8 {
                        significand >>= 1;
                        exponent = exponent.wrapping_add(1);
                    }
                }
            }

            exponent = exponent.saturating_add_signed(E);
            bits |= (exponent as u64) << 52;
            bits |= significand & 0xFFFFFFFFFFFFF;

            f64::from_bits(bits)
        }
    }

    /// Raw transmutation to [`u64`].
    #[must_use]
    pub const fn to_bits(self) -> u64 {
        self.0
    }

    /// Returns the memory representation of this fixed-point number as a byte array in native byte order.
    #[must_use]
    pub const fn to_ne_bytes(self) -> [u8; 8] {
        self.0.to_ne_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in big-endian (network) byte order.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in little-endian byte order.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    /// Returns the fixed-point significand, equal to `self` ⋅ 2<sup>-E</sup>.
    #[must_use]
    pub const fn significand(self) -> u64 {
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
    pub const fn add_signed(self, rhs: I64F<E>) -> Self {
        let x = self.0.wrapping_add(rhs.0 as u64);

        if cfg!(debug_assertions) && (rhs.0 < 0) != (x < self.0) {
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
    pub const fn strict_add_signed(self, rhs: I64F<E>) -> Self {
        let x = self.0.wrapping_add(rhs.0 as u64);

        if (rhs.0 < 0) != (x < self.0) {
            crate::panic::add();
        }

        Self(x)
    }

    /// Computes `self + rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_add_signed(self, rhs: I64F<E>) -> Self {
        Self(self.0.wrapping_add(rhs.0 as u64))
    }

    /// Computes `self + rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_add_signed(self, rhs: I64F<E>) -> Self {
        let x = self.0.wrapping_add(rhs.0 as u64);

        if (rhs.0 < 0) != (x > self.0) {
            if rhs.0 < 0 {
                return Self::MIN;
            } else {
                return Self::MAX;
            }
        }

        Self(x)
    }

    /// Computes `self + rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_add_signed(self, rhs: I64F<E>) -> (Self, bool) {
        let x = self.0.wrapping_add(rhs.0 as u64);

        (Self(x), (rhs.0 < 0) != (x < self.0))
    }

    /// Computes `self + rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_add_signed(self, rhs: I64F<E>) -> Option<Self> {
        let x = self.0.wrapping_add(rhs.0 as u64);

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
    pub const fn sub_signed(self, rhs: I64F<E>) -> Self {
        let x = self.0.wrapping_sub(rhs.0 as u64);

        if cfg!(debug_assertions) && (rhs.0 < 0) != (x > self.0) {
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
    pub const fn strict_sub_signed(self, rhs: I64F<E>) -> Self {
        let x = self.0.wrapping_sub(rhs.0 as u64);

        if (rhs.0 < 0) != (x > self.0) {
            crate::panic::sub();
        }

        Self(x)
    }

    /// Computes `self - rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_sub_signed(self, rhs: I64F<E>) -> Self {
        Self(self.0.wrapping_sub(rhs.0 as u64))
    }

    /// Computes `self - rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_sub_signed(self, rhs: I64F<E>) -> Self {
        let x = self.0.wrapping_sub(rhs.0 as u64);

        if (rhs.0 < 0) != (x > self.0) {
            if rhs.0 < 0 {
                return Self::MAX;
            } else {
                return Self::MIN;
            }
        }

        Self(x)
    }

    /// Computes `self - rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_sub_signed(self, rhs: I64F<E>) -> (Self, bool) {
        let x = self.0.wrapping_sub(rhs.0 as u64);

        (Self(x), (rhs.0 < 0) != (x > self.0))
    }

    /// Computes `self - rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_sub_signed(self, rhs: I64F<E>) -> Option<Self> {
        let x = self.0.wrapping_sub(rhs.0 as u64);

        if (rhs.0 < 0) != (x > self.0) {
            return None;
        }

        Some(Self(x))
    }
}

impl From<U64F<0>> for u64 {
    fn from(value: U64F<0>) -> Self {
        value.0
    }
}

impl From<u64> for U64F<0> {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl<const E: i32> From<U8F<E>> for U64F<E> {
    /// Converts from [`U8F<E>`] losslessly.
    fn from(value: U8F<E>) -> Self {
        Self::from_u8f(value)
    }
}

impl<const E: i32> From<U16F<E>> for U64F<E> {
    /// Converts from [`U16F<E>`] losslessly.
    fn from(value: U16F<E>) -> Self {
        Self::from_u16f(value)
    }
}

impl<const E: i32> From<U32F<E>> for U64F<E> {
    /// Converts from [`U32F<E>`] losslessly.
    fn from(value: U32F<E>) -> Self {
        Self::from_u32f(value)
    }
}

impl<const E1: i32, const E2: i32> PartialEq<U64F<E2>> for U64F<E1> {
    fn eq(&self, other: &U64F<E2>) -> bool {
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

impl<const E1: i32, const E2: i32> PartialOrd<U64F<E2>> for U64F<E1> {
    fn partial_cmp(&self, other: &U64F<E2>) -> Option<cmp::Ordering> {
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

impl<const E: i32> fmt::Debug for U64F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U64F<{E}")?;

        f.debug_tuple(">").field(&self.0).finish()
    }
}

impl<const E: i32> fmt::Binary for U64F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::Octal for U64F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::LowerHex for U64F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::UpperHex for U64F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> ops::Add for U64F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<const E: i32> ops::Sub for U64F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
