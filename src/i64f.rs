use ::core::cmp;
use ::core::fmt;
use ::core::ops;

use crate::I8F;
use crate::I16F;
use crate::I32F;
use crate::I128F;
use crate::U8F;
use crate::U16F;
use crate::U32F;
use crate::U64F;
use crate::U128F;
use crate::error::TryFromFloatError;

/// The 32-bit unsigned fixed-point type.
#[derive(Clone, Copy, Eq, Hash, Ord)]
pub struct I64F<const E: i32> {
    pub(crate) significand: i64,
}

impl I64F<-65> {
    /// 1/τ
    pub const FRAC_1_TAU: Self = Self {
        significand: 0x517CC1B727220A95,
    };
}

impl I64F<-64> {
    /// 1/π
    pub const FRAC_1_PI: Self = Self {
        significand: 0x517CC1B727220A95,
    };
    /// π/8
    pub const FRAC_PI_8: Self = Self {
        significand: 0x6487ED5110B4611A,
    };
    /// log<sub>10</sub>(2)
    pub const LOG10_2: Self = Self {
        significand: 0x4D104D427DE7FBCC,
    };
    /// log<sub>10</sub>(e)
    pub const LOG10_E: Self = Self {
        significand: 0x6F2DEC549B9438CB,
    };
}

impl I64F<-63> {
    /// The Euler-Mascheroni constant (γ)
    pub const EULER_GAMMA: Self = Self {
        significand: 0x49E233F1BED863D2,
    };
    /// 1/sqrt(2)
    pub const FRAC_1_SQRT_2: Self = Self {
        significand: 0x5A827999FCEF3242,
    };
    /// 2/π
    pub const FRAC_2_PI: Self = Self {
        significand: 0x517CC1B727220A95,
    };
    /// π/4
    pub const FRAC_PI_4: Self = Self {
        significand: 0x6487ED5110B4611A,
    };
    /// π/6
    pub const FRAC_PI_6: Self = Self {
        significand: 0x430548E0B5CD9612,
    };
    /// ln(2)
    pub const LN_2: Self = Self {
        significand: 0x58B90BFBE8E7BCD6,
    };
}

impl I64F<-62> {
    /// 2/sqrt(π)
    pub const FRAC_2_SQRT_PI: Self = Self {
        significand: 0x48375D410A6DB447,
    };
    /// π/2
    pub const FRAC_PI_2: Self = Self {
        significand: 0x6487ED5110B4611A,
    };
    /// π/3
    pub const FRAC_PI_3: Self = Self {
        significand: 0x430548E0B5CD9612,
    };
    /// The golden ratio (φ)
    pub const GOLDEN_RATIO: Self = Self {
        significand: 0x678DDE6E5FD29F05,
    };
    /// log<sub>2</sub>(e)
    pub const LOG2_E: Self = Self {
        significand: 0x5C551D94AE0BF85E,
    };
    /// sqrt(2)
    pub const SQRT_2: Self = Self {
        significand: 0x5A827999FCEF3242,
    };
}

impl I64F<-61> {
    /// Euler's number (e)
    pub const E: Self = Self {
        significand: 0x56FC2A2C515DA54D,
    };
    /// ln(10)
    pub const LN_10: Self = Self {
        significand: 0x49AEC6EED554560B,
    };
    /// log<sub>2</sub>(10)
    pub const LOG2_10: Self = Self {
        significand: 0x6A4D3C25E68DC57F,
    };
    /// Archimedes’ constant (π)
    pub const PI: Self = Self {
        significand: 0x6487ED5110B4611A,
    };
}

impl I64F<-60> {
    /// The full circle constant (τ)
    pub const TAU: Self = Self {
        significand: 0x6487ED5110B4611A,
    };
}

impl<const E: i32> I64F<E> {
    /// The smallest value that can be represented by this fixed-point type, equal to -2<sup>63</sup> ⋅ 2<sup>E</sup>.
    pub const MIN: Self = Self {
        significand: i64::MIN,
    };

    /// The largest value that can be represented by this fixed-point type, equal to (2<sup>63</sup> - 1) ⋅ 2<sup>E</sup>.
    pub const MAX: Self = Self {
        significand: i64::MAX,
    };

    /// The size of this type in bits.
    pub const BITS: u32 = i64::BITS;

    /// Creates a new fixed-point number from an integer significand, equal to `significand` ⋅ 2<sup>E</sup>.
    #[inline(always)]
    #[must_use]
    pub const fn new(significand: i64) -> Self {
        Self { significand }
    }

    /// Tries to create a new fixed-point number from [`f32`]. Returns the nearest multiple of 2<sup>E</sup> to `value`, rounded to the number with even least significant digits if `value` is halfway between two multiples of 2<sup>E</sup>. Returns an error if `value` is not a number, less than [`Self::MIN`], or greater than [`Self::MAX`].
    pub const fn try_new_from_f32(value: f32) -> Result<Self, TryFromFloatError> {
        let bits = value.to_bits();

        if bits & 0x7FFFFFFF == 0 {
            return Ok(Self { significand: 0 });
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

        let mut significand = significand as i64;

        if negative {
            significand = significand.wrapping_neg();
        }

        let exponent = (exponent as i32).wrapping_sub(const { 127 + 23 });

        if exponent >= E {
            let shift = exponent.wrapping_sub(E) as u32;

            if shift >= significand.leading_zeros() | significand.leading_ones() {
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

            if shift >= i64::BITS {
                significand = 0;
            } else {
                significand = significand.wrapping_add(significand >> shift & 0x1);
                significand = significand.wrapping_add(!(!0 << shift.wrapping_sub(1)));
                significand >>= shift;
            }
        }

        Ok(Self { significand })
    }

    /// Tries to create a new fixed-point number from [`f64`]. Returns the nearest multiple of 2<sup>E</sup> to `value`, rounded to the number with even least significant digits if `value` is halfway between two multiples of 2<sup>E</sup>. Returns an error if `value` is not a number, less than [`Self::MIN`], or greater than [`Self::MAX`].
    pub const fn try_new_from_f64(value: f64) -> Result<Self, TryFromFloatError> {
        let bits = value.to_bits();

        if bits & 0x7FFFFFFFFFFFFFFF == 0 {
            return Ok(Self { significand: 0 });
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

        let mut significand = significand as i64;

        if negative {
            significand = significand.wrapping_neg();
        }

        let exponent = (exponent as i32).wrapping_sub(const { 1023 + 52 });

        if exponent >= E {
            let shift = exponent.wrapping_sub(E) as u32;

            if shift >= significand.leading_zeros() | significand.leading_ones() {
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

            if shift >= i64::BITS {
                significand = 0;
            } else {
                significand = significand.wrapping_add(significand >> shift & 0x1);
                significand = significand.wrapping_add(!(!0 << shift.wrapping_sub(1)));
                significand >>= shift;
            }
        }

        Ok(Self { significand })
    }

    /// Converts from [`I8F<E>`] losslessly.
    #[must_use]
    pub const fn from_i8f(value: I8F<E>) -> Self {
        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`I16F<E>`] losslessly.
    #[must_use]
    pub const fn from_i16f(value: I16F<E>) -> Self {
        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`I32F<E>`] losslessly.
    #[must_use]
    pub const fn from_i32f(value: I32F<E>) -> Self {
        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`I128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_i128f(value: I128F<E>) -> Self {
        if cfg!(debug_assertions)
            && (value.significand < i64::MIN as i128 || value.significand > i64::MAX as i128)
        {
            crate::panic::from();
        }

        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`I128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_i128f(value: I128F<E>) -> Self {
        if value.significand < i64::MIN as i128 || value.significand > i64::MAX as i128 {
            crate::panic::from();
        }

        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`I128F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_i128f(value: I128F<E>) -> Self {
        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`I128F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_i128f(value: I128F<E>) -> Self {
        if value.significand < i64::MIN as i128 {
            return Self::MIN;
        } else if value.significand > i64::MAX as i128 {
            return Self::MAX;
        }

        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`I128F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_i128f(value: I128F<E>) -> (Self, bool) {
        let overflowed =
            value.significand < i64::MIN as i128 || value.significand > i64::MAX as i128;
        let significand = value.significand as i64;

        (Self { significand }, overflowed)
    }

    /// Converts from [`I128F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_i128f(value: I128F<E>) -> Option<Self> {
        if value.significand < i64::MIN as i128 || value.significand > i64::MAX as i128 {
            return None;
        }

        let significand = value.significand as i64;

        Some(Self { significand })
    }

    /// Converts from [`U8F<E>`] losslessly.
    #[must_use]
    pub const fn from_u8f(value: U8F<E>) -> Self {
        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U16F<E>`] losslessly.
    #[must_use]
    pub const fn from_u16f(value: U16F<E>) -> Self {
        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U32F<E>`] losslessly.
    #[must_use]
    pub const fn from_u32f(value: U32F<E>) -> Self {
        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U64F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_u64f(value: U64F<E>) -> Self {
        if cfg!(debug_assertions) && value.significand > i64::MAX as u64 {
            crate::panic::from();
        }

        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U64F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_u64f(value: U64F<E>) -> Self {
        if value.significand > i64::MAX as u64 {
            crate::panic::from();
        }

        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U64F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_u64f(value: U64F<E>) -> Self {
        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U64F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_u64f(value: U64F<E>) -> Self {
        if value.significand > i64::MAX as u64 {
            return Self::MAX;
        }

        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U64F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_u64f(value: U64F<E>) -> (Self, bool) {
        let overflowed = value.significand > i64::MAX as u64;
        let significand = value.significand as i64;

        (Self { significand }, overflowed)
    }

    /// Converts from [`U64F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_u64f(value: U64F<E>) -> Option<Self> {
        if value.significand > i64::MAX as u64 {
            return None;
        }

        let significand = value.significand as i64;

        Some(Self { significand })
    }

    /// Converts from [`U128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_u128f(value: U128F<E>) -> Self {
        if cfg!(debug_assertions) && value.significand > i64::MAX as u128 {
            crate::panic::from();
        }

        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_u128f(value: U128F<E>) -> Self {
        if value.significand > i64::MAX as u128 {
            crate::panic::from();
        }

        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U128F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_u128f(value: U128F<E>) -> Self {
        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U128F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_u128f(value: U128F<E>) -> Self {
        if value.significand > i64::MAX as u128 {
            return Self::MAX;
        }

        let significand = value.significand as i64;

        Self { significand }
    }

    /// Converts from [`U128F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_u128f(value: U128F<E>) -> (Self, bool) {
        let overflowed = value.significand > i64::MAX as u128;
        let significand = value.significand as i64;

        (Self { significand }, overflowed)
    }

    /// Converts from [`U128F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_u128f(value: U128F<E>) -> Option<Self> {
        if value.significand > i64::MAX as u128 {
            return None;
        }

        let significand = value.significand as i64;

        Some(Self { significand })
    }

    /// Raw transutation from [`u64`].
    #[inline(always)]
    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self {
            significand: bits as i64,
        }
    }

    /// Creates a native endian fixed-point number from its memory representation as a byte array in native endian byte order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes), as appropriate, instead.
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; 8]) -> Self {
        Self {
            significand: i64::from_ne_bytes(bytes),
        }
    }

    /// Creates a fixed-point number from its memory representation as a byte array in big endian byte order.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self {
            significand: i64::from_be_bytes(bytes),
        }
    }

    /// Creates a fixed-point number from its memory representation as a byte array in little endian byte order.
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Self {
            significand: i64::from_le_bytes(bytes),
        }
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

            if scaling_factor == f32::INFINITY && self.significand == 0 {
                0.0
            } else {
                self.significand as f32 * scaling_factor
            }
        } else {
            let mut bits = 0;
            let mut significand = self.significand as u64;

            if self.significand < 0 {
                bits |= 0x80000000;
                significand = significand.wrapping_neg();
            }

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

            if scaling_factor == f64::INFINITY && self.significand == 0 {
                0.0
            } else {
                self.significand as f64 * scaling_factor
            }
        } else {
            let mut bits = 0;
            let mut significand = self.significand as u64;

            if self.significand < 0 {
                bits |= 0x8000000000000000;
                significand = significand.wrapping_neg();
            }

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

    /// Converts into [`I8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn into_i8f(self) -> I8F<E> {
        I8F::from_i64f(self)
    }

    /// Converts into [`I8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn strict_into_i8f(self) -> I8F<E> {
        I8F::strict_from_i64f(self)
    }

    /// Converts into [`I8F`], wrapping around at the numeric bounds of the type.
    #[inline(always)]
    #[must_use]
    pub const fn wrapping_into_i8f(self) -> I8F<E> {
        I8F::wrapping_from_i64f(self)
    }

    /// Converts into [`I8F`], saturating at the numeric bounds of the type instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub const fn saturating_into_i8f(self) -> I8F<E> {
        I8F::saturating_from_i64f(self)
    }

    /// Converts into [`I8F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn overflowing_into_i8f(self) -> (I8F<E>, bool) {
        I8F::overflowing_from_i64f(self)
    }

    /// Converts into [`I8F`], returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn checked_into_i8f(self) -> Option<I8F<E>> {
        I8F::checked_from_i64f(self)
    }

    /// Converts into [`I16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn into_i16f(self) -> I16F<E> {
        I16F::from_i64f(self)
    }

    /// Converts into [`I16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn strict_into_i16f(self) -> I16F<E> {
        I16F::strict_from_i64f(self)
    }

    /// Converts into [`I16F`], wrapping around at the numeric bounds of the type.
    #[inline(always)]
    #[must_use]
    pub const fn wrapping_into_i16f(self) -> I16F<E> {
        I16F::wrapping_from_i64f(self)
    }

    /// Converts into [`I16F`], saturating at the numeric bounds of the type instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub const fn saturating_into_i16f(self) -> I16F<E> {
        I16F::saturating_from_i64f(self)
    }

    /// Converts into [`I16F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn overflowing_into_i16f(self) -> (I16F<E>, bool) {
        I16F::overflowing_from_i64f(self)
    }

    /// Converts into [`I16F`], returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn checked_into_i16f(self) -> Option<I16F<E>> {
        I16F::checked_from_i64f(self)
    }

    /// Converts into [`I32F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn into_i32f(self) -> I32F<E> {
        I32F::from_i64f(self)
    }

    /// Converts into [`I32F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn strict_into_i32f(self) -> I32F<E> {
        I32F::strict_from_i64f(self)
    }

    /// Converts into [`I32F`], wrapping around at the numeric bounds of the type.
    #[inline(always)]
    #[must_use]
    pub const fn wrapping_into_i32f(self) -> I32F<E> {
        I32F::wrapping_from_i64f(self)
    }

    /// Converts into [`I32F`], saturating at the numeric bounds of the type instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub const fn saturating_into_i32f(self) -> I32F<E> {
        I32F::saturating_from_i64f(self)
    }

    /// Converts into [`I32F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn overflowing_into_i32f(self) -> (I32F<E>, bool) {
        I32F::overflowing_from_i64f(self)
    }

    /// Converts into [`I32F`], returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn checked_into_i32f(self) -> Option<I32F<E>> {
        I32F::checked_from_i64f(self)
    }

    /// Converts into [`I128F`] losslessly.
    #[inline(always)]
    #[must_use]
    pub const fn into_i128f(self) -> I128F<E> {
        I128F::from_i64f(self)
    }

    /// Converts into [`U8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn into_u8f(self) -> U8F<E> {
        U8F::from_i64f(self)
    }

    /// Converts into [`U8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u8f(self) -> U8F<E> {
        U8F::strict_from_i64f(self)
    }

    /// Converts into [`U8F`], wrapping around at the numeric bounds of the type.
    #[inline(always)]
    #[must_use]
    pub const fn wrapping_into_u8f(self) -> U8F<E> {
        U8F::wrapping_from_i64f(self)
    }

    /// Converts into [`U8F`], saturating at the numeric bounds of the type instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub const fn saturating_into_u8f(self) -> U8F<E> {
        U8F::saturating_from_i64f(self)
    }

    /// Converts into [`U8F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn overflowing_into_u8f(self) -> (U8F<E>, bool) {
        U8F::overflowing_from_i64f(self)
    }

    /// Converts into [`U8F`], returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn checked_into_u8f(self) -> Option<U8F<E>> {
        U8F::checked_from_i64f(self)
    }

    /// Converts into [`U16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn into_u16f(self) -> U16F<E> {
        U16F::from_i64f(self)
    }

    /// Converts into [`U16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u16f(self) -> U16F<E> {
        U16F::strict_from_i64f(self)
    }

    /// Converts into [`U16F`], wrapping around at the numeric bounds of the type.
    #[inline(always)]
    #[must_use]
    pub const fn wrapping_into_u16f(self) -> U16F<E> {
        U16F::wrapping_from_i64f(self)
    }

    /// Converts into [`U16F`], saturating at the numeric bounds of the type instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub const fn saturating_into_u16f(self) -> U16F<E> {
        U16F::saturating_from_i64f(self)
    }

    /// Converts into [`U16F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn overflowing_into_u16f(self) -> (U16F<E>, bool) {
        U16F::overflowing_from_i64f(self)
    }

    /// Converts into [`U16F`], returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn checked_into_u16f(self) -> Option<U16F<E>> {
        U16F::checked_from_i64f(self)
    }

    /// Converts into [`U32F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn into_u32f(self) -> U32F<E> {
        U32F::from_i64f(self)
    }

    /// Converts into [`U32F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u32f(self) -> U32F<E> {
        U32F::strict_from_i64f(self)
    }

    /// Converts into [`U32F`], wrapping around at the numeric bounds of the type.
    #[inline(always)]
    #[must_use]
    pub const fn wrapping_into_u32f(self) -> U32F<E> {
        U32F::wrapping_from_i64f(self)
    }

    /// Converts into [`U32F`], saturating at the numeric bounds of the type instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub const fn saturating_into_u32f(self) -> U32F<E> {
        U32F::saturating_from_i64f(self)
    }

    /// Converts into [`U32F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn overflowing_into_u32f(self) -> (U32F<E>, bool) {
        U32F::overflowing_from_i64f(self)
    }

    /// Converts into [`U32F`], returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn checked_into_u32f(self) -> Option<U32F<E>> {
        U32F::checked_from_i64f(self)
    }

    /// Converts into [`U64F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn into_u64f(self) -> U64F<E> {
        U64F::from_i64f(self)
    }

    /// Converts into [`U64F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u64f(self) -> U64F<E> {
        U64F::strict_from_i64f(self)
    }

    /// Converts into [`U64F`], wrapping around at the numeric bounds of the type.
    #[inline(always)]
    #[must_use]
    pub const fn wrapping_into_u64f(self) -> U64F<E> {
        U64F::wrapping_from_i64f(self)
    }

    /// Converts into [`U64F`], saturating at the numeric bounds of the type instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub const fn saturating_into_u64f(self) -> U64F<E> {
        U64F::saturating_from_i64f(self)
    }

    /// Converts into [`U64F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn overflowing_into_u64f(self) -> (U64F<E>, bool) {
        U64F::overflowing_from_i64f(self)
    }

    /// Converts into [`U64F`], returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn checked_into_u64f(self) -> Option<U64F<E>> {
        U64F::checked_from_i64f(self)
    }

    /// Converts into [`U128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn into_u128f(self) -> U128F<E> {
        U128F::from_i64f(self)
    }

    /// Converts into [`U128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[inline(always)]
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u128f(self) -> U128F<E> {
        U128F::strict_from_i64f(self)
    }

    /// Converts into [`U128F`], wrapping around at the numeric bounds of the type.
    #[inline(always)]
    #[must_use]
    pub const fn wrapping_into_u128f(self) -> U128F<E> {
        U128F::wrapping_from_i64f(self)
    }

    /// Converts into [`U128F`], saturating at the numeric bounds of the type instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub const fn saturating_into_u128f(self) -> U128F<E> {
        U128F::saturating_from_i64f(self)
    }

    /// Converts into [`U128F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn overflowing_into_u128f(self) -> (U128F<E>, bool) {
        U128F::overflowing_from_i64f(self)
    }

    /// Converts into [`U128F`], returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub const fn checked_into_u128f(self) -> Option<U128F<E>> {
        U128F::checked_from_i64f(self)
    }

    /// Raw transmutation to [`u64`].
    #[inline(always)]
    #[must_use]
    pub const fn to_bits(self) -> u64 {
        self.significand as u64
    }

    /// Returns the memory representation of this fixed-point number as a byte array in native byte order.
    #[must_use]
    pub const fn to_ne_bytes(self) -> [u8; 8] {
        self.significand.to_ne_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in big-endian (network) byte order.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.significand.to_be_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in little-endian byte order.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 8] {
        self.significand.to_le_bytes()
    }

    /// Returns the fixed-point significand, equal to `self` ⋅ 2<sup>-E</sup>.
    #[inline(always)]
    #[must_use]
    pub const fn significand(self) -> i64 {
        self.significand
    }

    /// Returns the fixed-point exponent.
    #[inline(always)]
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
        let x = self.significand.ilog2();
        let Some(x) = E.checked_add_unsigned(x) else {
            crate::panic::ilog2();
        };

        x
    }

    /// Computes the base 2 logarithm of `self`, rounded down. Returns `None` if `self` is zero, or if overflow occurred.
    #[must_use]
    pub const fn checked_ilog2(self) -> Option<i32> {
        let Some(x) = self.significand.checked_ilog2() else {
            return None;
        };
        let Some(x) = E.checked_add_unsigned(x) else {
            return None;
        };

        Some(x)
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn rescale<const E2: i32>(self) -> I64F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::rescale();
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u64;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I64F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_rescale<const E2: i32>(self) -> I64F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::rescale();
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u64;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I64F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_rescale<const E2: i32>(self) -> I64F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u64;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I64F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_rescale<const E2: i32>(self) -> I64F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                if x < 0 {
                    return I64F::MIN;
                } else {
                    return I64F::MAX;
                }
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u64;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I64F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_rescale<const E2: i32>(self) -> (I64F<E2>, bool) {
        let mut x = self.significand;
        let mut overflowed = false;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            overflowed |= x != 0 && shift >= x.leading_zeros() | x.leading_ones();

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u64;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        (I64F { significand: x }, overflowed)
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_rescale<const E2: i32>(self) -> Option<I64F<E2>> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                return None;
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u64;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        Some(I64F { significand: x })
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn neg(self) -> Self {
        Self {
            significand: -self.significand,
        }
    }

    /// Computes `-self`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_neg(self) -> Self {
        Self {
            significand: self.significand.strict_neg(),
        }
    }

    /// Computes `-self`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_neg(self) -> Self {
        Self {
            significand: self.significand.wrapping_neg(),
        }
    }

    /// Computes `-self`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_neg(self) -> Self {
        Self {
            significand: self.significand.saturating_neg(),
        }
    }

    /// Computes `-self`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        let (x, overflow) = self.significand.overflowing_neg();

        (Self { significand: x }, overflow)
    }

    /// Computes `-self`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_neg(self) -> Option<Self> {
        let Some(x) = self.significand.checked_neg() else {
            return None;
        };

        Some(Self { significand: x })
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn add(self, rhs: Self) -> Self {
        Self {
            significand: self.significand + rhs.significand,
        }
    }

    /// Computes `self + rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_add(self, rhs: Self) -> Self {
        Self {
            significand: self.significand.strict_add(rhs.significand),
        }
    }

    /// Computes `self + rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self {
            significand: self.significand.wrapping_add(rhs.significand),
        }
    }

    /// Computes `self + rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self {
            significand: self.significand.saturating_add(rhs.significand),
        }
    }

    /// Computes `self + rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (x, overflowed) = self.significand.overflowing_add(rhs.significand);

        (Self { significand: x }, overflowed)
    }

    /// Computes `self + rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        let Some(x) = self.significand.checked_add(rhs.significand) else {
            return None;
        };

        Some(Self { significand: x })
    }

    /// Computes `self + rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn add_unsigned(self, rhs: U64F<E>) -> Self {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        if cfg!(debug_assertions) && x < self.significand {
            crate::panic::add();
        }

        Self { significand: x }
    }

    /// Computes `self + rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_add_unsigned(self, rhs: U64F<E>) -> Self {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        if x < self.significand {
            crate::panic::add();
        }

        Self { significand: x }
    }

    /// Computes `self + rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_add_unsigned(self, rhs: U64F<E>) -> Self {
        Self {
            significand: self.significand.wrapping_add_unsigned(rhs.significand),
        }
    }

    /// Computes `self + rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_add_unsigned(self, rhs: U64F<E>) -> Self {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        if x < self.significand {
            return Self::MAX;
        }

        Self { significand: x }
    }

    /// Computes `self + rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_add_unsigned(self, rhs: U64F<E>) -> (Self, bool) {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        (Self { significand: x }, x < self.significand)
    }

    /// Computes `self + rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_add_unsigned(self, rhs: U64F<E>) -> Option<Self> {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        if x < self.significand {
            return None;
        }

        Some(Self { significand: x })
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn sub(self, rhs: Self) -> Self {
        Self {
            significand: self.significand - rhs.significand,
        }
    }

    /// Computes `self - rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_sub(self, rhs: Self) -> Self {
        Self {
            significand: self.significand.strict_sub(rhs.significand),
        }
    }

    /// Computes `self - rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self {
            significand: self.significand.wrapping_sub(rhs.significand),
        }
    }

    /// Computes `self - rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self {
            significand: self.significand.saturating_sub(rhs.significand),
        }
    }

    /// Computes `self - rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (x, overflowed) = self.significand.overflowing_sub(rhs.significand);

        (Self { significand: x }, overflowed)
    }

    /// Computes `self - rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        let Some(x) = self.significand.checked_sub(rhs.significand) else {
            return None;
        };

        Some(Self { significand: x })
    }

    /// Computes `self - rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn sub_unsigned(self, rhs: U64F<E>) -> Self {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        if cfg!(debug_assertions) && x > self.significand {
            crate::panic::sub();
        }

        Self { significand: x }
    }

    /// Computes `self - rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_sub_unsigned(self, rhs: U64F<E>) -> Self {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        if x > self.significand {
            crate::panic::sub();
        }

        Self { significand: x }
    }

    /// Computes `self - rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_sub_unsigned(self, rhs: U64F<E>) -> Self {
        Self {
            significand: self.significand.wrapping_sub_unsigned(rhs.significand),
        }
    }

    /// Computes `self - rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_sub_unsigned(self, rhs: U64F<E>) -> Self {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        if x > self.significand {
            return Self::MIN;
        }

        Self { significand: x }
    }

    /// Computes `self - rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_sub_unsigned(self, rhs: U64F<E>) -> (Self, bool) {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        (Self { significand: x }, x > self.significand)
    }

    /// Computes `self - rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_sub_unsigned(self, rhs: U64F<E>) -> Option<Self> {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        if x > self.significand {
            return None;
        }

        Some(Self { significand: x })
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn mul<const R: i32>(self, rhs: I64F<R>) -> Self {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::mul();
            }

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if cfg!(debug_assertions) && (x < i64::MIN as i128 || x > i64::MAX as i128) {
            crate::panic::mul();
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self * rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_mul<const R: i32>(self, rhs: I64F<R>) -> Self {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::mul();
            }

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i64::MIN as i128 || x > i64::MAX as i128 {
            crate::panic::mul();
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self * rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_mul<const R: i32>(self, rhs: I64F<R>) -> Self {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self * rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_mul<const R: i32>(self, rhs: I64F<R>) -> Self {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                if x < 0 {
                    return Self::MIN;
                } else {
                    return Self::MAX;
                }
            }

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i64::MIN as i128 {
            return Self::MIN;
        } else if x > i64::MAX as i128 {
            return Self::MAX;
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self * rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_mul<const R: i32>(self, rhs: I64F<R>) -> (Self, bool) {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);
        let mut overflowed = false;

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            overflowed |= x != 0 && shift >= x.leading_zeros() | x.leading_ones();

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        overflowed |= x < i64::MIN as i128 || x > i64::MAX as i128;

        (
            Self {
                significand: x as i64,
            },
            overflowed,
        )
    }

    /// Computes `self * rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_mul<const R: i32>(self, rhs: I64F<R>) -> Option<Self> {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                return None;
            }

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i64::MIN as i128 || x > i64::MAX as i128 {
            return None;
        }

        Some(Self {
            significand: x as i64,
        })
    }

    /// Computes `self * rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn mul_unsigned<const R: i32>(self, rhs: U64F<R>) -> Self {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::mul();
            }

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u128;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        if cfg!(debug_assertions) && (x < i64::MIN as i128 || x > i64::MAX as i128) {
            crate::panic::mul();
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self * rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_mul_unsigned<const R: i32>(self, rhs: U64F<R>) -> Self {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::mul();
            }

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u128;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        if x < i64::MIN as i128 || x > i64::MAX as i128 {
            crate::panic::mul();
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self * rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_mul_unsigned<const R: i32>(self, rhs: U64F<R>) -> Self {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u128;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self * rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_mul_unsigned<const R: i32>(self, rhs: U64F<R>) -> Self {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                if x < 0 {
                    return Self::MIN;
                } else {
                    return Self::MAX;
                }
            }

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u128;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        if x < i64::MIN as i128 {
            return Self::MIN;
        } else if x > i64::MAX as i128 {
            return Self::MAX;
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self * rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_mul_unsigned<const R: i32>(self, rhs: U64F<R>) -> (Self, bool) {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);
        let mut overflowed = false;

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            overflowed |= x != 0 && shift >= x.leading_zeros() | x.leading_ones();

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u128;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        overflowed |= x < i64::MIN as i128 || x > i64::MAX as i128;

        (
            Self {
                significand: x as i64,
            },
            overflowed,
        )
    }

    /// Computes `self * rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_mul_unsigned<const R: i32>(self, rhs: U64F<R>) -> Option<Self> {
        let mut x = (self.significand as i128).wrapping_mul(rhs.significand as i128);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                return None;
            }

            if shift >= i128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i128::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u128;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        if x < i64::MIN as i128 || x > i64::MAX as i128 {
            return None;
        }

        Some(Self {
            significand: x as i64,
        })
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn div<const R: i32>(self, rhs: I64F<R>) -> Self {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i128) << -OFFSET).overflowing_div(rhs.significand as i128);
        let negative = (x < 0) != overflowed;
        let mut x = x as u128;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift > x.leading_zeros() {
                crate::panic::div();
            }

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if cfg!(debug_assertions) && x > i64::MAX as u128 + negative as u128 {
            crate::panic::div();
        }

        if negative {
            x = x.wrapping_neg();
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self / rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    ///
    /// ## Overflow behavior
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_div<const R: i32>(self, rhs: I64F<R>) -> Self {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i128) << -OFFSET).overflowing_div(rhs.significand as i128);
        let negative = (x < 0) != overflowed;
        let mut x = x as u128;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() {
                crate::panic::div();
            }

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x > (i64::MAX as u128).wrapping_add(negative as u128) {
            crate::panic::div();
        }

        if negative {
            x = x.wrapping_neg();
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self / rhs`, wrapping around at the numeric bounds of the type.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn wrapping_div<const R: i32>(self, rhs: I64F<R>) -> Self {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i128) << -OFFSET).overflowing_div(rhs.significand as i128);
        let negative = (x < 0) != overflowed;
        let mut x = x as u128;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if negative {
            x = x.wrapping_neg();
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self / rhs`, saturating at the numeric bounds of the type instead of overflowing.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn saturating_div<const R: i32>(self, rhs: I64F<R>) -> Self {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i128) << -OFFSET).overflowing_div(rhs.significand as i128);
        let negative = (x < 0) != overflowed;
        let mut x = x as u128;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() {
                if negative {
                    return Self::MIN;
                } else {
                    return Self::MAX;
                }
            }

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x > (i64::MAX as u128).wrapping_add(negative as u128) {
            if negative {
                return Self::MIN;
            } else {
                return Self::MAX;
            }
        }

        if negative {
            x = x.wrapping_neg();
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self / rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn overflowing_div<const R: i32>(self, rhs: I64F<R>) -> (Self, bool) {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i128) << -OFFSET).overflowing_div(rhs.significand as i128);
        let negative = (x < 0) != overflowed;
        let mut x = x as u128;
        let mut overflowed = false;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            overflowed |= x != 0 && shift > x.leading_zeros();

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        overflowed |= x > (i64::MAX as u128).wrapping_add(negative as u128);

        if negative {
            x = x.wrapping_neg();
        }

        (
            Self {
                significand: x as i64,
            },
            overflowed,
        )
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or overflow occurred.
    #[must_use]
    pub const fn checked_div<const R: i32>(self, rhs: I64F<R>) -> Option<Self> {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        if rhs.significand == 0 {
            return None;
        }

        let (x, overflowed) =
            ((self.significand as i128) << -OFFSET).overflowing_div(rhs.significand as i128);
        let negative = (x < 0) != overflowed;
        let mut x = x as u128;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() {
                return None;
            }

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x > (i64::MAX as u128).wrapping_add(negative as u128) {
            return None;
        }

        if negative {
            x = x.wrapping_neg();
        }

        Some(Self {
            significand: x as i64,
        })
    }

    /// Computes `self / rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    ///
    /// ## Overflow behavior
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn div_unsigned<const R: i32>(self, rhs: U64F<R>) -> Self {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        let mut x = ((self.significand as i128) << -OFFSET) / rhs.significand as i128;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                crate::panic::div();
            }

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if cfg!(debug_assertions) && (x < i64::MIN as i128 || x > i64::MAX as i128) {
            crate::panic::div();
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self / rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    ///
    /// ## Overflow behavior
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_div_unsigned<const R: i32>(self, rhs: U64F<R>) -> Self {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        let mut x = ((self.significand as i128) << -OFFSET) / rhs.significand as i128;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                crate::panic::div();
            }

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i64::MIN as i128 || x > i64::MAX as i128 {
            crate::panic::div();
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self / rhs`, wrapping around at the numeric bounds of the type.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn wrapping_div_unsigned<const R: i32>(self, rhs: U64F<R>) -> Self {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        let mut x = ((self.significand as i128) << -OFFSET) / rhs.significand as i128;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self / rhs`, saturating at the numeric bounds of the type instead of overflowing.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn saturating_div_unsigned<const R: i32>(self, rhs: U64F<R>) -> Self {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        let mut x = ((self.significand as i128) << -OFFSET) / rhs.significand as i128;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                if x < 0 {
                    return Self::MIN;
                } else {
                    return Self::MAX;
                }
            }

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i64::MIN as i128 {
            return Self::MIN;
        } else if x > i64::MAX as i128 {
            return Self::MAX;
        }

        Self {
            significand: x as i64,
        }
    }

    /// Computes `self / rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn overflowing_div_unsigned<const R: i32>(self, rhs: U64F<R>) -> (Self, bool) {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        let mut x = ((self.significand as i128) << -OFFSET) / rhs.significand as i128;
        let mut overflowed = false;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            overflowed |= x != 0 && shift > x.leading_zeros() | x.leading_ones();

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        overflowed |= x < i64::MIN as i128 || x > i64::MAX as i128;

        (
            Self {
                significand: x as i64,
            },
            overflowed,
        )
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or overflow occurred.
    #[must_use]
    pub const fn checked_div_unsigned<const R: i32>(self, rhs: U64F<R>) -> Option<Self> {
        const OFFSET: i32 = i64::BITS.cast_signed() - i128::BITS.cast_signed();

        if rhs.significand == 0 {
            return None;
        }

        let mut x = ((self.significand as i128) << -OFFSET) / rhs.significand as i128;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                return None;
            }

            if shift >= u128::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u128::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i128).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i64::MIN as i128 || x > i64::MAX as i128 {
            return None;
        }

        Some(Self {
            significand: x as i64,
        })
    }
}

impl I64F<-63> {
    /// Computes `cos(π * self)` using a minimax second-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.60096 ⋅ 10<sup>-2</sup>.
    #[must_use]
    pub const fn cospi_2(self) -> I64F<-62> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i64_2(theta);

        I64F { significand }
    }

    /// Computes `cos(π * self)` using a minimax fourth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.18799 ⋅ 10<sup>-4</sup>.
    #[must_use]
    pub const fn cospi_4(self) -> I64F<-62> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i64_4(theta);

        I64F { significand }
    }

    /// Computes `cos(π * self)` using a minimax sixth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.20285 ⋅ 10<sup>-6</sup>.
    #[must_use]
    pub const fn cospi_6(self) -> I64F<-62> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i64_6(theta);

        I64F { significand }
    }

    /// Computes `cos(π * self)` using a minimax eighth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.98045 ⋅ 10<sup>-8</sup>.
    #[must_use]
    pub const fn cospi_8(self) -> I64F<-62> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i64_8(theta);

        I64F { significand }
    }

    /// Computes `cos(π * self)` using a minimax tenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 2.70068 ⋅ 10<sup>-10</sup>.
    #[must_use]
    pub const fn cospi_10(self) -> I64F<-62> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i64_10(theta);

        I64F { significand }
    }

    /// Computes `cos(π * self)` using a minimax twelfth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 8.93703 ⋅ 10<sup>-13</sup>.
    #[must_use]
    pub const fn cospi_12(self) -> I64F<-62> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i64_12(theta);

        I64F { significand }
    }

    /// Computes `cos(π * self)` using a minimax fourteenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 2.25676 ⋅ 10<sup>-15</sup>.
    #[must_use]
    pub const fn cospi_14(self) -> I64F<-62> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i64_14(theta);

        I64F { significand }
    }

    /// Computes `cos(π * self)` using a minimax sixteenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 4.48780 ⋅ 10<sup>-18</sup>.
    #[must_use]
    pub const fn cospi_16(self) -> I64F<-62> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i64_16(theta);

        I64F { significand }
    }

    /// Computes `cos(π * self)` using a minimax eighteenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 7.20675 ⋅ 10<sup>-21</sup>.
    #[must_use]
    pub const fn cospi_18(self) -> I64F<-62> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i64_18(theta);

        I64F { significand }
    }

    /// Computes `sin(π * self)` using a minimax second-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.60096 ⋅ 10<sup>-2</sup>.
    #[must_use]
    pub const fn sinpi_2(self) -> I64F<-62> {
        const PHASE_SHIFT: i64 = 0x3 << (i64::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i64_2(theta);

        I64F { significand }
    }

    /// Computes `sin(π * self)` using a minimax fourth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.18799 ⋅ 10<sup>-4</sup>.
    #[must_use]
    pub const fn sinpi_4(self) -> I64F<-62> {
        const PHASE_SHIFT: i64 = 0x3 << (i64::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i64_4(theta);

        I64F { significand }
    }

    /// Computes `sin(π * self)` using a minimax sixth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.20285 ⋅ 10<sup>-6</sup>.
    #[must_use]
    pub const fn sinpi_6(self) -> I64F<-62> {
        const PHASE_SHIFT: i64 = 0x3 << (i64::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i64_6(theta);

        I64F { significand }
    }

    /// Computes `sin(π * self)` using a minimax eighth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.98045 ⋅ 10<sup>-8</sup>.
    #[must_use]
    pub const fn sinpi_8(self) -> I64F<-62> {
        const PHASE_SHIFT: i64 = 0x3 << (i64::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i64_8(theta);

        I64F { significand }
    }

    /// Computes `sin(π * self)` using a minimax tenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 2.70068 ⋅ 10<sup>-10</sup>.
    #[must_use]
    pub const fn sinpi_10(self) -> I64F<-62> {
        const PHASE_SHIFT: i64 = 0x3 << (i64::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i64_10(theta);

        I64F { significand }
    }

    /// Computes `sin(π * self)` using a minimax twelfth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 8.93703 ⋅ 10<sup>-13</sup>.
    #[must_use]
    pub const fn sinpi_12(self) -> I64F<-62> {
        const PHASE_SHIFT: i64 = 0x3 << (i64::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i64_12(theta);

        I64F { significand }
    }

    /// Computes `sin(π * self)` using a minimax fourteenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 2.25676 ⋅ 10<sup>-15</sup>.
    #[must_use]
    pub const fn sinpi_14(self) -> I64F<-62> {
        const PHASE_SHIFT: i64 = 0x3 << (i64::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i64_14(theta);

        I64F { significand }
    }

    /// Computes `sin(π * self)` using a minimax sixteenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 4.48780 ⋅ 10<sup>-18</sup>.
    #[must_use]
    pub const fn sinpi_16(self) -> I64F<-62> {
        const PHASE_SHIFT: i64 = 0x3 << (i64::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i64_16(theta);

        I64F { significand }
    }

    /// Computes `sin(π * self)` using a minimax eighteenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 7.20675 ⋅ 10<sup>-21</sup>.
    #[must_use]
    pub const fn sinpi_18(self) -> I64F<-62> {
        const PHASE_SHIFT: i64 = 0x3 << (i64::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i64_18(theta);

        I64F { significand }
    }
}

impl From<I64F<0>> for i64 {
    fn from(value: I64F<0>) -> Self {
        value.significand
    }
}

impl From<i64> for I64F<0> {
    fn from(value: i64) -> Self {
        Self { significand: value }
    }
}

impl<const E: i32> From<I8F<E>> for I64F<E> {
    fn from(value: I8F<E>) -> Self {
        Self::from_i8f(value)
    }
}

impl<const E: i32> From<I16F<E>> for I64F<E> {
    fn from(value: I16F<E>) -> Self {
        Self::from_i16f(value)
    }
}

impl<const E: i32> From<I32F<E>> for I64F<E> {
    fn from(value: I32F<E>) -> Self {
        Self::from_i32f(value)
    }
}

impl<const E: i32> From<U8F<E>> for I64F<E> {
    fn from(value: U8F<E>) -> Self {
        Self::from_u8f(value)
    }
}

impl<const E: i32> From<U16F<E>> for I64F<E> {
    fn from(value: U16F<E>) -> Self {
        Self::from_u16f(value)
    }
}

impl<const E: i32> From<U32F<E>> for I64F<E> {
    fn from(value: U32F<E>) -> Self {
        Self::from_u32f(value)
    }
}

impl<const E1: i32, const E2: i32> PartialEq<I64F<E2>> for I64F<E1> {
    fn eq(&self, other: &I64F<E2>) -> bool {
        let mut lhs = self.significand;
        let mut rhs = other.significand;

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

impl<const E1: i32, const E2: i32> PartialOrd<I64F<E2>> for I64F<E1> {
    fn partial_cmp(&self, other: &I64F<E2>) -> Option<cmp::Ordering> {
        let mut lhs = self.significand;
        let mut rhs = other.significand;

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

impl<const E: i32> fmt::Debug for I64F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I64F<{E}")?;

        f.debug_tuple(">").field(&self.significand).finish()
    }
}

impl<const E: i32> fmt::Binary for I64F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::Octal for I64F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::LowerHex for I64F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::UpperHex for I64F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> ops::Neg for I64F<E> {
    type Output = Self;

    #[track_caller]
    fn neg(self) -> Self::Output {
        Self::neg(self)
    }
}

impl<const E: i32> ops::Add for I64F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self::add(self, rhs)
    }
}

impl<const E: i32> ops::Add<U64F<E>> for I64F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: U64F<E>) -> Self::Output {
        Self::add_unsigned(self, rhs)
    }
}

impl<const E: i32> ops::Sub for I64F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::sub(self, rhs)
    }
}

impl<const E: i32> ops::Sub<U64F<E>> for I64F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: U64F<E>) -> Self::Output {
        Self::sub_unsigned(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Mul<I64F<R>> for I64F<E> {
    type Output = Self;

    #[track_caller]
    fn mul(self, rhs: I64F<R>) -> Self::Output {
        Self::mul(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Mul<U64F<R>> for I64F<E> {
    type Output = Self;

    #[track_caller]
    fn mul(self, rhs: U64F<R>) -> Self::Output {
        Self::mul_unsigned(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Div<I64F<R>> for I64F<E> {
    type Output = Self;

    #[track_caller]
    fn div(self, rhs: I64F<R>) -> Self::Output {
        Self::div(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Div<U64F<R>> for I64F<E> {
    type Output = Self;

    #[track_caller]
    fn div(self, rhs: U64F<R>) -> Self::Output {
        Self::div_unsigned(self, rhs)
    }
}

impl<const E: i32> ops::AddAssign for I64F<E> {
    #[track_caller]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(*self, rhs)
    }
}

impl<const E: i32> ops::AddAssign<U64F<E>> for I64F<E> {
    #[track_caller]
    fn add_assign(&mut self, rhs: U64F<E>) {
        *self = Self::add_unsigned(*self, rhs)
    }
}

impl<const E: i32> ops::SubAssign for I64F<E> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::sub(*self, rhs)
    }
}

impl<const E: i32> ops::SubAssign<U64F<E>> for I64F<E> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: U64F<E>) {
        *self = Self::sub_unsigned(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::MulAssign<I64F<R>> for I64F<E> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: I64F<R>) {
        *self = Self::mul(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::MulAssign<U64F<R>> for I64F<E> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: U64F<R>) {
        *self = Self::mul_unsigned(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::DivAssign<I64F<R>> for I64F<E> {
    #[track_caller]
    fn div_assign(&mut self, rhs: I64F<R>) {
        *self = Self::div(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::DivAssign<U64F<R>> for I64F<E> {
    #[track_caller]
    fn div_assign(&mut self, rhs: U64F<R>) {
        *self = Self::div_unsigned(*self, rhs)
    }
}
