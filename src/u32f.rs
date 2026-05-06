use ::core::cmp;
use ::core::fmt;
use ::core::ops;

use crate::I8F;
use crate::I16F;
use crate::I32F;
use crate::I64F;
use crate::I128F;
use crate::U8F;
use crate::U16F;
use crate::U64F;
use crate::U128F;
use crate::error::TryFromFloatError;

/// The 32-bit unsigned fixed-point type.
#[derive(Clone, Copy, Eq, Ord)]
pub struct U32F<const E: i32> {
    pub(crate) significand: u32,
}

impl U32F<-34> {
    /// 1/τ
    pub const FRAC_1_TAU: Self = Self {
        significand: 0xA2F9836E,
    };
}

impl U32F<-33> {
    /// 1/π
    pub const FRAC_1_PI: Self = Self {
        significand: 0xA2F9836E,
    };
    /// π/8
    pub const FRAC_PI_8: Self = Self {
        significand: 0xC90FDAA2,
    };
    /// log<sub>10</sub>(2)
    pub const LOG10_2: Self = Self {
        significand: 0x9A209A85,
    };
    /// log<sub>10</sub>(e)
    pub const LOG10_E: Self = Self {
        significand: 0xDE5BD8A9,
    };
}

impl U32F<-32> {
    /// The Euler-Mascheroni constant (γ)
    pub const EULER_GAMMA: Self = Self {
        significand: 0x93C467E3,
    };
    /// 1/sqrt(2)
    pub const FRAC_1_SQRT_2: Self = Self {
        significand: 0xB504F334,
    };
    /// 2/π
    pub const FRAC_2_PI: Self = Self {
        significand: 0xA2F9836E,
    };
    /// π/4
    pub const FRAC_PI_4: Self = Self {
        significand: 0xC90FDAA2,
    };
    /// π/6
    pub const FRAC_PI_6: Self = Self {
        significand: 0x860A91C1,
    };
    /// ln(2)
    pub const LN_2: Self = Self {
        significand: 0xB17217F8,
    };
}

impl U32F<-31> {
    /// 2/sqrt(π)
    pub const FRAC_2_SQRT_PI: Self = Self {
        significand: 0x906EBA82,
    };
    /// π/2
    pub const FRAC_PI_2: Self = Self {
        significand: 0xC90FDAA2,
    };
    /// π/3
    pub const FRAC_PI_3: Self = Self {
        significand: 0x860A91C1,
    };
    /// The golden ratio (φ)
    pub const GOLDEN_RATIO: Self = Self {
        significand: 0xCF1BBCDD,
    };
    /// log<sub>2</sub>(e)
    pub const LOG2_E: Self = Self {
        significand: 0xB8AA3B29,
    };
    /// sqrt(2)
    pub const SQRT_2: Self = Self {
        significand: 0xB504F334,
    };
}

impl U32F<-30> {
    /// Euler's number (e)
    pub const E: Self = Self {
        significand: 0xADF85459,
    };
    /// ln(10)
    pub const LN_10: Self = Self {
        significand: 0x935D8DDE,
    };
    /// log<sub>2</sub>(10)
    pub const LOG2_10: Self = Self {
        significand: 0xD49A784C,
    };
    /// Archimedes’ constant (π)
    pub const PI: Self = Self {
        significand: 0xC90FDAA2,
    };
}

impl U32F<-29> {
    /// The full circle constant (τ)
    pub const TAU: Self = Self {
        significand: 0xC90FDAA2,
    };
}

impl<const E: i32> U32F<E> {
    /// The smallest value that can be represented by this fixed-point type, equal to 0.
    pub const MIN: Self = Self {
        significand: u32::MIN,
    };

    /// The largest value that can be represented by this fixed-point type, equal to (2<sup>32</sup> - 1) ⋅ 2<sup>E</sup>.
    pub const MAX: Self = Self {
        significand: u32::MAX,
    };

    /// The size of this type in bits.
    pub const BITS: u32 = u32::BITS;

    /// Creates a new fixed-point number from an integer significand, equal to `significand` ⋅ 2<sup>E</sup>.
    #[must_use]
    pub const fn new(significand: u32) -> Self {
        Self { significand }
    }

    /// Tries to convert from [`f32`]. Returns the nearest multiple of 2<sup>E</sup> to `value`, rounded to the number with even least significant digits if `value` is halfway between two multiples of 2<sup>E</sup>. Returns an error if `value` is not a number, less than [`Self::MIN`], or greater than [`Self::MAX`].
    pub const fn try_from_f32(value: f32) -> Result<Self, TryFromFloatError> {
        const EXPONENT_BIAS: i32 = !(!0 << (EXPONENT_BITS - 1));
        const EXPONENT_BITS: u32 = 8;
        const EXPONENT_MASK: u32 = !(!0 << EXPONENT_BITS);
        const EXPONENT_SHIFT: u32 = SIGNIFICAND_BITS;
        const IMPLICIT_BIT: u32 = 1 << SIGNIFICAND_BITS;
        const SIGN_SHIFT: u32 = EXPONENT_BITS + SIGNIFICAND_BITS;
        const SIGNIFICAND_BITS: u32 = 23;
        const SIGNIFICAND_MASK: u32 = !(!0 << SIGNIFICAND_BITS);
        const ZERO_MASK: u32 = !(!0 << SIGN_SHIFT);

        let bits = value.to_bits();

        if bits & ZERO_MASK == 0 {
            return Ok(Self { significand: 0 });
        }

        let mut significand = bits & SIGNIFICAND_MASK;
        let mut exponent = bits >> EXPONENT_SHIFT & EXPONENT_MASK;
        let sign = bits >> SIGN_SHIFT;

        if exponent == EXPONENT_MASK {
            if significand != 0 {
                return Err(TryFromFloatError::Nan);
            } else {
                return Err(TryFromFloatError::Overflow);
            }
        } else if exponent > 0 {
            significand |= IMPLICIT_BIT;
        } else {
            exponent = 1;
        }

        let exponent = exponent as i32 - const { EXPONENT_BIAS + SIGNIFICAND_BITS.cast_signed() };

        if exponent >= E {
            let shift = exponent.wrapping_sub(E).cast_unsigned();
            let temp = significand.unbounded_shl(shift);

            if temp.unbounded_shr(shift) != significand {
                return Err(TryFromFloatError::Overflow);
            }

            significand = temp;
        } else {
            let shift = E.wrapping_sub(exponent).cast_unsigned();

            if shift > const { SIGNIFICAND_BITS + 1 } {
                significand = 0;
            } else {
                significand += significand >> shift & 0x1;
                significand += !(!0 << (shift - 1));
                significand >>= shift;
            }
        }

        if sign != 0 && significand > 0 {
            return Err(TryFromFloatError::Overflow);
        }

        Ok(Self { significand })
    }

    /// Tries to convert from [`f64`]. Returns the nearest multiple of 2<sup>E</sup> to `value`, rounded to the number with even least significant digits if `value` is halfway between two multiples of 2<sup>E</sup>. Returns an error if `value` is not a number, less than [`Self::MIN`], or greater than [`Self::MAX`].
    pub const fn try_from_f64(value: f64) -> Result<Self, TryFromFloatError> {
        const EXPONENT_BIAS: i32 = !(!0 << (EXPONENT_BITS - 1));
        const EXPONENT_BITS: u32 = 11;
        const EXPONENT_MASK: u64 = !(!0 << EXPONENT_BITS);
        const EXPONENT_SHIFT: u32 = SIGNIFICAND_BITS;
        const IMPLICIT_BIT: u64 = 1 << SIGNIFICAND_BITS;
        const SIGN_SHIFT: u32 = EXPONENT_BITS + SIGNIFICAND_BITS;
        const SIGNIFICAND_BITS: u32 = 52;
        const SIGNIFICAND_MASK: u64 = !(!0 << SIGNIFICAND_BITS);
        const ZERO_MASK: u64 = !(!0 << SIGN_SHIFT);

        let bits = value.to_bits();

        if bits & ZERO_MASK == 0 {
            return Ok(Self { significand: 0 });
        }

        let mut significand = bits & SIGNIFICAND_MASK;
        let mut exponent = bits >> EXPONENT_SHIFT & EXPONENT_MASK;
        let sign = bits >> SIGN_SHIFT;

        if exponent == EXPONENT_MASK {
            if significand != 0 {
                return Err(TryFromFloatError::Nan);
            } else {
                return Err(TryFromFloatError::Overflow);
            }
        } else if exponent > 0 {
            significand |= IMPLICIT_BIT;
        } else {
            exponent = 1;
        }

        let exponent = exponent as i32 - const { EXPONENT_BIAS + SIGNIFICAND_BITS.cast_signed() };

        if exponent >= E {
            let shift = exponent.wrapping_sub(E).cast_unsigned();
            let temp = significand.unbounded_shl(shift);

            if temp.unbounded_shr(shift) != significand {
                return Err(TryFromFloatError::Overflow);
            }

            significand = temp;
        } else {
            let shift = E.wrapping_sub(exponent).cast_unsigned();

            if shift > const { SIGNIFICAND_BITS + 1 } {
                significand = 0;
            } else {
                significand += significand >> shift & 0x1;
                significand += !(!0 << (shift - 1));
                significand >>= shift;
            }
        }

        if sign != 0 && significand > 0 || significand > u32::MAX as u64 {
            return Err(TryFromFloatError::Overflow);
        }

        let significand = significand as u32;

        Ok(Self { significand })
    }

    /// Converts from [`I8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_i8f(value: I8F<E>) -> Self {
        match Self::overflowing_from_i8f(value) {
            (_, true) if cfg!(debug_assertions) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`I8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_i8f(value: I8F<E>) -> Self {
        match Self::overflowing_from_i8f(value) {
            (_, true) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`I8F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_i8f(value: I8F<E>) -> Option<Self> {
        match Self::overflowing_from_i8f(value) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Converts from [`I8F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn overflowing_from_i8f(value: I8F<E>) -> (Self, bool) {
        let overflowed = value.significand.is_negative();
        let significand = value.significand as u32;

        (Self { significand }, overflowed)
    }

    /// Converts from [`I8F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_i8f(value: I8F<E>) -> Self {
        Self::overflowing_from_i8f(value).0
    }

    /// Converts from [`I8F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn saturating_from_i8f(value: I8F<E>) -> Self {
        match Self::overflowing_from_i8f(value) {
            (_, true) => Self::MIN,
            (x, _) => x,
        }
    }

    /// Converts from [`I16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_i16f(value: I16F<E>) -> Self {
        match Self::overflowing_from_i16f(value) {
            (_, true) if cfg!(debug_assertions) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`I16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_i16f(value: I16F<E>) -> Self {
        match Self::overflowing_from_i16f(value) {
            (_, true) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`I16F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_i16f(value: I16F<E>) -> Option<Self> {
        match Self::overflowing_from_i16f(value) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Converts from [`I16F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn overflowing_from_i16f(value: I16F<E>) -> (Self, bool) {
        let overflowed = value.significand.is_negative();
        let significand = value.significand as u32;

        (Self { significand }, overflowed)
    }

    /// Converts from [`I16F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_i16f(value: I16F<E>) -> Self {
        Self::overflowing_from_i16f(value).0
    }

    /// Converts from [`I16F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn saturating_from_i16f(value: I16F<E>) -> Self {
        match Self::overflowing_from_i16f(value) {
            (_, true) => {
                if value.significand.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
            (x, _) => x,
        }
    }

    /// Converts from [`I32F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_i32f(value: I32F<E>) -> Self {
        match Self::overflowing_from_i32f(value) {
            (_, true) if cfg!(debug_assertions) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`I32F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_i32f(value: I32F<E>) -> Self {
        match Self::overflowing_from_i32f(value) {
            (_, true) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`I32F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_i32f(value: I32F<E>) -> Option<Self> {
        match Self::overflowing_from_i32f(value) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Converts from [`I32F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_i32f(value: I32F<E>) -> (Self, bool) {
        let overflowed = value.significand.is_negative();
        let significand = value.significand as u32;

        (Self { significand }, overflowed)
    }

    /// Converts from [`I32F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_i32f(value: I32F<E>) -> Self {
        Self::overflowing_from_i32f(value).0
    }

    /// Converts from [`I32F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_i32f(value: I32F<E>) -> Self {
        match Self::overflowing_from_i32f(value) {
            (_, true) => {
                if value.significand.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
            (x, _) => x,
        }
    }

    /// Converts from [`I64F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_i64f(value: I64F<E>) -> Self {
        match Self::overflowing_from_i64f(value) {
            (_, true) if cfg!(debug_assertions) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`I64F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_i64f(value: I64F<E>) -> Self {
        match Self::overflowing_from_i64f(value) {
            (_, true) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`I64F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_i64f(value: I64F<E>) -> Option<Self> {
        match Self::overflowing_from_i64f(value) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Converts from [`I64F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_i64f(value: I64F<E>) -> (Self, bool) {
        let overflowed = value.significand.is_negative() || value.significand > u32::MAX as i64;
        let significand = value.significand as u32;

        (Self { significand }, overflowed)
    }

    /// Converts from [`I64F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_i64f(value: I64F<E>) -> Self {
        Self::overflowing_from_i64f(value).0
    }

    /// Converts from [`I64F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_i64f(value: I64F<E>) -> Self {
        match Self::overflowing_from_i64f(value) {
            (_, true) => {
                if value.significand.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
            (x, _) => x,
        }
    }

    /// Converts from [`I128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_i128f(value: I128F<E>) -> Self {
        match Self::overflowing_from_i128f(value) {
            (_, true) if cfg!(debug_assertions) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`I128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_i128f(value: I128F<E>) -> Self {
        match Self::overflowing_from_i128f(value) {
            (_, true) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`I128F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_i128f(value: I128F<E>) -> Option<Self> {
        match Self::overflowing_from_i128f(value) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Converts from [`I128F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_i128f(value: I128F<E>) -> (Self, bool) {
        let overflowed = value.significand.is_negative() || value.significand > u32::MAX as i128;
        let significand = value.significand as u32;

        (Self { significand }, overflowed)
    }

    /// Converts from [`I128F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_i128f(value: I128F<E>) -> Self {
        Self::overflowing_from_i128f(value).0
    }

    /// Converts from [`I128F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_i128f(value: I128F<E>) -> Self {
        match Self::overflowing_from_i128f(value) {
            (_, true) => {
                if value.significand.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
            (x, _) => x,
        }
    }

    /// Converts from [`U8F<E>`] losslessly.
    #[must_use]
    pub const fn from_u8f(value: U8F<E>) -> Self {
        let significand = value.significand as u32;

        Self { significand }
    }

    /// Converts from [`U16F<E>`] losslessly.
    #[must_use]
    pub const fn from_u16f(value: U16F<E>) -> Self {
        let significand = value.significand as u32;

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
        match Self::overflowing_from_u64f(value) {
            (_, true) if cfg!(debug_assertions) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`U64F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_u64f(value: U64F<E>) -> Self {
        match Self::overflowing_from_u64f(value) {
            (_, true) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`U64F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_u64f(value: U64F<E>) -> Option<Self> {
        match Self::overflowing_from_u64f(value) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Converts from [`U64F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_u64f(value: U64F<E>) -> (Self, bool) {
        let overflowed = value.significand > u32::MAX as u64;
        let significand = value.significand as u32;

        (Self { significand }, overflowed)
    }

    /// Converts from [`U64F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_u64f(value: U64F<E>) -> Self {
        Self::overflowing_from_u64f(value).0
    }

    /// Converts from [`U64F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_u64f(value: U64F<E>) -> Self {
        match Self::overflowing_from_u64f(value) {
            (_, true) => Self::MAX,
            (x, _) => x,
        }
    }

    /// Converts from [`U128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_u128f(value: U128F<E>) -> Self {
        match Self::overflowing_from_u128f(value) {
            (_, true) if cfg!(debug_assertions) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`U128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_u128f(value: U128F<E>) -> Self {
        match Self::overflowing_from_u128f(value) {
            (_, true) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`U128F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_u128f(value: U128F<E>) -> Option<Self> {
        match Self::overflowing_from_u128f(value) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Converts from [`U128F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_u128f(value: U128F<E>) -> (Self, bool) {
        let overflowed = value.significand > u32::MAX as u128;
        let significand = value.significand as u32;

        (Self { significand }, overflowed)
    }

    /// Converts from [`U128F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_u128f(value: U128F<E>) -> Self {
        Self::overflowing_from_u128f(value).0
    }

    /// Converts from [`U128F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_u128f(value: U128F<E>) -> Self {
        match Self::overflowing_from_u128f(value) {
            (_, true) => Self::MAX,
            (x, _) => x,
        }
    }

    /// Raw transutation from [`u32`].
    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        Self { significand: bits }
    }

    /// Creates a native endian fixed-point number from its memory representation as a byte array in native endian byte order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes), as appropriate, instead.
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; 4]) -> Self {
        Self {
            significand: u32::from_ne_bytes(bytes),
        }
    }

    /// Creates a fixed-point number from its memory representation as a byte array in big endian byte order.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self {
            significand: u32::from_be_bytes(bytes),
        }
    }

    /// Creates a fixed-point number from its memory representation as a byte array in little endian byte order.
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self {
            significand: u32::from_le_bytes(bytes),
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
            let mut significand = self.significand;

            let leading_zeros = significand.leading_zeros();
            let mut exponent = const { BIAS + u32::BITS - 1 }.wrapping_sub(leading_zeros);
            let mut align = const { u32::BITS - f32::MANTISSA_DIGITS };
            align = align.wrapping_add(leading_zeros.saturating_sub_signed(
                const { E.saturating_add_unsigned((BIAS - 1) + (u32::BITS - 1)) },
            ));

            if leading_zeros >= align {
                let shift = leading_zeros.wrapping_sub(align);
                significand <<= shift;
            } else {
                let shift = align.wrapping_sub(leading_zeros);

                if shift >= u32::BITS {
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
            bits |= significand & 0x7FFFFF;

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
    #[must_use]
    #[track_caller]
    pub const fn into_i8f(self) -> I8F<E> {
        I8F::from_u32f(self)
    }

    /// Converts into [`I8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_into_i8f(self) -> I8F<E> {
        I8F::strict_from_u32f(self)
    }

    /// Converts into [`I8F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_into_i8f(self) -> Option<I8F<E>> {
        I8F::checked_from_u32f(self)
    }

    /// Converts into [`I8F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_into_i8f(self) -> (I8F<E>, bool) {
        I8F::overflowing_from_u32f(self)
    }

    /// Converts into [`I8F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_into_i8f(self) -> I8F<E> {
        I8F::wrapping_from_u32f(self)
    }

    /// Converts into [`I8F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_into_i8f(self) -> I8F<E> {
        I8F::saturating_from_u32f(self)
    }

    /// Converts into [`I16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn into_i16f(self) -> I16F<E> {
        I16F::from_u32f(self)
    }

    /// Converts into [`I16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_into_i16f(self) -> I16F<E> {
        I16F::strict_from_u32f(self)
    }

    /// Converts into [`I16F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_into_i16f(self) -> Option<I16F<E>> {
        I16F::checked_from_u32f(self)
    }

    /// Converts into [`I16F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_into_i16f(self) -> (I16F<E>, bool) {
        I16F::overflowing_from_u32f(self)
    }

    /// Converts into [`I16F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_into_i16f(self) -> I16F<E> {
        I16F::wrapping_from_u32f(self)
    }

    /// Converts into [`I16F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_into_i16f(self) -> I16F<E> {
        I16F::saturating_from_u32f(self)
    }

    /// Converts into [`I64F`] losslessly.
    #[must_use]
    #[track_caller]
    pub const fn into_i64f(self) -> I64F<E> {
        I64F::from_u32f(self)
    }

    /// Converts into [`I128F`] losslessly.
    #[must_use]
    #[track_caller]
    pub const fn into_i128f(self) -> I128F<E> {
        I128F::from_u32f(self)
    }

    /// Converts into [`U8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn into_u8f(self) -> U8F<E> {
        U8F::from_u32f(self)
    }

    /// Converts into [`U8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u8f(self) -> U8F<E> {
        U8F::strict_from_u32f(self)
    }

    /// Converts into [`U8F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_into_u8f(self) -> Option<U8F<E>> {
        U8F::checked_from_u32f(self)
    }

    /// Converts into [`U8F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_into_u8f(self) -> (U8F<E>, bool) {
        U8F::overflowing_from_u32f(self)
    }

    /// Converts into [`U8F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_into_u8f(self) -> U8F<E> {
        U8F::wrapping_from_u32f(self)
    }

    /// Converts into [`U8F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_into_u8f(self) -> U8F<E> {
        U8F::saturating_from_u32f(self)
    }

    /// Converts into [`U16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn into_u16f(self) -> U16F<E> {
        U16F::from_u32f(self)
    }

    /// Converts into [`U16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u16f(self) -> U16F<E> {
        U16F::strict_from_u32f(self)
    }

    /// Converts into [`U16F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_into_u16f(self) -> Option<U16F<E>> {
        U16F::checked_from_u32f(self)
    }

    /// Converts into [`U16F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_into_u16f(self) -> (U16F<E>, bool) {
        U16F::overflowing_from_u32f(self)
    }

    /// Converts into [`U16F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_into_u16f(self) -> U16F<E> {
        U16F::wrapping_from_u32f(self)
    }

    /// Converts into [`U16F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_into_u16f(self) -> U16F<E> {
        U16F::saturating_from_u32f(self)
    }

    /// Converts into [`U64F`] losslessly.
    #[must_use]
    #[track_caller]
    pub const fn into_u64f(self) -> U64F<E> {
        U64F::from_u32f(self)
    }

    /// Converts into [`U128F`] losslessly.
    #[must_use]
    #[track_caller]
    pub const fn into_u128f(self) -> U128F<E> {
        U128F::from_u32f(self)
    }

    /// Raw transmutation to [`u32`].
    #[must_use]
    pub const fn to_bits(self) -> u32 {
        self.significand
    }

    /// Returns the memory representation of this fixed-point number as a byte array in native byte order.
    #[must_use]
    pub const fn to_ne_bytes(self) -> [u8; 4] {
        self.significand.to_ne_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in big-endian (network) byte order.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.significand.to_be_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in little-endian byte order.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 4] {
        self.significand.to_le_bytes()
    }

    /// Returns the fixed-point significand, equal to `self` ⋅ 2<sup>-E</sup>.
    #[must_use]
    pub const fn significand(self) -> u32 {
        self.significand
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
    pub const fn rescale<const E2: i32>(self) -> U32F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift > x.leading_zeros() {
                crate::panic::rescale();
            }

            if shift >= u32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= u32::BITS {
                x = 0;
            } else if shift == u32::BITS {
                let threshold = const { 1 << (u32::BITS - 1) };

                x = (x > threshold) as u32;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = (x & mask).wrapping_add(x >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add(temp);
            }
        }

        U32F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_rescale<const E2: i32>(self) -> U32F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() {
                crate::panic::rescale();
            }

            if shift >= u32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= u32::BITS {
                x = 0;
            } else if shift == u32::BITS {
                let threshold = const { 1 << (u32::BITS - 1) };

                x = (x > threshold) as u32;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = (x & mask).wrapping_add(x >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add(temp);
            }
        }

        U32F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_rescale<const E2: i32>(self) -> U32F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if shift >= u32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= u32::BITS {
                x = 0;
            } else if shift == u32::BITS {
                let threshold = const { 1 << (u32::BITS - 1) };

                x = (x > threshold) as u32;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = (x & mask).wrapping_add(x >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add(temp);
            }
        }

        U32F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_rescale<const E2: i32>(self) -> U32F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() {
                return U32F::MAX;
            }

            if shift >= u32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= u32::BITS {
                x = 0;
            } else if shift == u32::BITS {
                let threshold = const { 1 << (u32::BITS - 1) };

                x = (x > threshold) as u32;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = (x & mask).wrapping_add(x >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add(temp);
            }
        }

        U32F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_rescale<const E2: i32>(self) -> (U32F<E2>, bool) {
        let mut x = self.significand;
        let mut overflowed = false;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            overflowed |= x != 0 && shift > x.leading_zeros();

            if shift >= u32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= u32::BITS {
                x = 0;
            } else if shift == u32::BITS {
                let threshold = const { 1 << (u32::BITS - 1) };

                x = (x > threshold) as u32;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = (x & mask).wrapping_add(x >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add(temp);
            }
        }

        (U32F { significand: x }, overflowed)
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_rescale<const E2: i32>(self) -> Option<U32F<E2>> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() {
                return None;
            }

            if shift >= u32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= u32::BITS {
                x = 0;
            } else if shift == u32::BITS {
                let threshold = const { 1 << (u32::BITS - 1) };

                x = (x > threshold) as u32;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = (x & mask).wrapping_add(x >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add(temp);
            }
        }

        Some(U32F { significand: x })
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
    pub const fn add_signed(self, rhs: I32F<E>) -> Self {
        let x = self.significand.wrapping_add(rhs.significand as u32);

        if cfg!(debug_assertions) && (rhs.significand < 0) != (x < self.significand) {
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
    pub const fn strict_add_signed(self, rhs: I32F<E>) -> Self {
        let x = self.significand.wrapping_add(rhs.significand as u32);

        if (rhs.significand < 0) != (x < self.significand) {
            crate::panic::add();
        }

        Self { significand: x }
    }

    /// Computes `self + rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_add_signed(self, rhs: I32F<E>) -> Self {
        Self {
            significand: self.significand.wrapping_add(rhs.significand as u32),
        }
    }

    /// Computes `self + rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_add_signed(self, rhs: I32F<E>) -> Self {
        let x = self.significand.wrapping_add(rhs.significand as u32);

        if (rhs.significand < 0) != (x > self.significand) {
            if rhs.significand < 0 {
                return Self::MIN;
            } else {
                return Self::MAX;
            }
        }

        Self { significand: x }
    }

    /// Computes `self + rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_add_signed(self, rhs: I32F<E>) -> (Self, bool) {
        let x = self.significand.wrapping_add(rhs.significand as u32);

        (
            Self { significand: x },
            (rhs.significand < 0) != (x < self.significand),
        )
    }

    /// Computes `self + rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_add_signed(self, rhs: I32F<E>) -> Option<Self> {
        let x = self.significand.wrapping_add(rhs.significand as u32);

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
    pub const fn sub_signed(self, rhs: I32F<E>) -> Self {
        let x = self.significand.wrapping_sub(rhs.significand as u32);

        if cfg!(debug_assertions) && (rhs.significand < 0) != (x > self.significand) {
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
    pub const fn strict_sub_signed(self, rhs: I32F<E>) -> Self {
        let x = self.significand.wrapping_sub(rhs.significand as u32);

        if (rhs.significand < 0) != (x > self.significand) {
            crate::panic::sub();
        }

        Self { significand: x }
    }

    /// Computes `self - rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_sub_signed(self, rhs: I32F<E>) -> Self {
        Self {
            significand: self.significand.wrapping_sub(rhs.significand as u32),
        }
    }

    /// Computes `self - rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_sub_signed(self, rhs: I32F<E>) -> Self {
        let x = self.significand.wrapping_sub(rhs.significand as u32);

        if (rhs.significand < 0) != (x > self.significand) {
            if rhs.significand < 0 {
                return Self::MAX;
            } else {
                return Self::MIN;
            }
        }

        Self { significand: x }
    }

    /// Computes `self - rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_sub_signed(self, rhs: I32F<E>) -> (Self, bool) {
        let x = self.significand.wrapping_sub(rhs.significand as u32);

        (
            Self { significand: x },
            (rhs.significand < 0) != (x > self.significand),
        )
    }

    /// Computes `self - rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_sub_signed(self, rhs: I32F<E>) -> Option<Self> {
        let x = self.significand.wrapping_sub(rhs.significand as u32);

        if (rhs.significand < 0) != (x > self.significand) {
            return None;
        }

        Some(Self { significand: x })
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn mul<const R: i32>(self, rhs: U32F<R>) -> Self {
        match self.overflowing_mul(rhs) {
            (_, true) if cfg!(debug_assertions) => crate::panic::mul(),
            (x, _) => x,
        }
    }

    /// Computes `self * rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_mul<const R: i32>(self, rhs: U32F<R>) -> Self {
        match self.overflowing_mul(rhs) {
            (_, true) => crate::panic::mul(),
            (x, _) => x,
        }
    }

    /// Computes `self * rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_mul<const R: i32>(self, rhs: U32F<R>) -> Option<Self> {
        match self.overflowing_mul(rhs) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Computes `self * rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_mul<const R: i32>(self, rhs: U32F<R>) -> (Self, bool) {
        let mut significand = self.significand as u64 * rhs.significand as u64;
        let mut overflowed = false;

        if R >= 0 {
            let shift = R.cast_unsigned();
            let temp = significand.unbounded_shl(shift);
            overflowed |= temp.unbounded_shr(shift) != significand;
            significand = temp;
        } else {
            let shift = R.wrapping_neg().cast_unsigned();

            if shift > u64::BITS {
                significand = 0;
            } else if shift == u64::BITS {
                const THRESHOLD: u64 = 1 << (u64::BITS - 1);

                significand = (significand > THRESHOLD) as u64;
            } else {
                let mut temp = significand;
                let mask = !(!0 << shift);
                let round = !(!0 << (shift - 1));
                temp = (temp & mask) + round + (temp >> shift & 0x1);
                temp >>= shift;
                significand >>= shift;
                significand += temp;
            }
        }

        overflowed |= significand > u32::MAX as u64;
        let significand = significand as u32;

        (Self { significand }, overflowed)
    }

    /// Computes `self * rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_mul<const R: i32>(self, rhs: U32F<R>) -> Self {
        self.overflowing_mul(rhs).0
    }

    /// Computes `self * rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_mul<const R: i32>(self, rhs: U32F<R>) -> Self {
        match self.overflowing_mul(rhs) {
            (_, true) => Self::MAX,
            (x, _) => x,
        }
    }

    /// Computes `self * rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn mul_signed<const R: i32>(self, rhs: I32F<R>) -> Self {
        match self.overflowing_mul_signed(rhs) {
            (_, true) if cfg!(debug_assertions) => crate::panic::mul(),
            (x, _) => x,
        }
    }

    /// Computes `self * rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_mul_signed<const R: i32>(self, rhs: I32F<R>) -> Self {
        match self.overflowing_mul_signed(rhs) {
            (_, true) => crate::panic::mul(),
            (x, _) => x,
        }
    }

    /// Computes `self * rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_mul_signed<const R: i32>(self, rhs: I32F<R>) -> Option<Self> {
        match self.overflowing_mul_signed(rhs) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Computes `self * rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_mul_signed<const R: i32>(self, rhs: I32F<R>) -> (Self, bool) {
        let mut significand = self.significand as i64 * rhs.significand as i64;
        let mut overflowed = false;

        if R >= 0 {
            let shift = R.cast_unsigned();
            let temp = significand.unbounded_shl(shift);
            overflowed |= temp.unbounded_shr(shift) != significand;
            significand = temp;
        } else {
            let shift = R.wrapping_neg().cast_unsigned();

            if shift >= i64::BITS {
                significand = 0;
            } else {
                let mut temp = significand.cast_unsigned();
                let mask = !(!0 << shift);
                let round = !(!0 << (shift - 1));
                temp = (temp & mask) + round + (temp >> shift & 0x1);
                temp >>= shift;
                significand >>= shift;
                significand += temp.cast_signed();
            }
        }

        overflowed |= significand < u32::MIN as i64 || significand > u32::MAX as i64;
        let significand = significand as u32;

        (Self { significand }, overflowed)
    }

    /// Computes `self * rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_mul_signed<const R: i32>(self, rhs: I32F<R>) -> Self {
        self.overflowing_mul_signed(rhs).0
    }

    /// Computes `self * rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_mul_signed<const R: i32>(self, rhs: I32F<R>) -> Self {
        match self.overflowing_mul_signed(rhs) {
            (_, true) => {
                if rhs.significand.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
            (x, _) => x,
        }
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn div<const R: i32>(self, rhs: U32F<R>) -> Self {
        match self.overflowing_div(rhs) {
            (_, true) if cfg!(debug_assertions) => crate::panic::div(),
            (x, _) => x,
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
    pub const fn strict_div<const R: i32>(self, rhs: U32F<R>) -> Self {
        match self.overflowing_div(rhs) {
            (_, true) => crate::panic::div(),
            (x, _) => x,
        }
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or overflow occurred.
    #[must_use]
    pub const fn checked_div<const R: i32>(self, rhs: U32F<R>) -> Option<Self> {
        match self.overflowing_div(rhs) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Computes `self / rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn overflowing_div<const R: i32>(self, rhs: U32F<R>) -> (Self, bool) {
        const OFFSET: i32 = u32::BITS.cast_signed() - u64::BITS.cast_signed();

        let mut x = ((self.significand as u64) << -OFFSET) / rhs.significand as u64;
        let mut overflowed = false;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            overflowed |= x != 0 && shift > x.leading_zeros();

            if shift >= u64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = (x & mask).wrapping_add(x >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add(temp);
            }
        }

        overflowed |= x > u32::MAX as u64;

        (
            Self {
                significand: x as u32,
            },
            overflowed,
        )
    }

    /// Computes `self / rhs`, wrapping around at the numeric bounds of the type.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn wrapping_div<const R: i32>(self, rhs: U32F<R>) -> Self {
        self.overflowing_div(rhs).0
    }

    /// Computes `self / rhs`, saturating at the numeric bounds of the type instead of overflowing.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn saturating_div<const R: i32>(self, rhs: U32F<R>) -> Self {
        match self.overflowing_div(rhs) {
            (_, true) => Self::MAX,
            (x, _) => x,
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
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn div_signed<const R: i32>(self, rhs: I32F<R>) -> Self {
        match self.overflowing_div_signed(rhs) {
            (_, true) if cfg!(debug_assertions) => crate::panic::div(),
            (x, _) => x,
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
    pub const fn strict_div_signed<const R: i32>(self, rhs: I32F<R>) -> Self {
        match self.overflowing_div_signed(rhs) {
            (_, true) => crate::panic::div(),
            (x, _) => x,
        }
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or overflow occurred.
    #[must_use]
    pub const fn checked_div_signed<const R: i32>(self, rhs: I32F<R>) -> Option<Self> {
        match self.overflowing_div_signed(rhs) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Computes `self / rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn overflowing_div_signed<const R: i32>(self, rhs: I32F<R>) -> (Self, bool) {
        const OFFSET: i32 = u32::BITS.cast_signed() - u64::BITS.cast_signed();

        let negative = rhs.significand < 0;
        let mut rhs = rhs.significand as u32;

        if negative {
            rhs = rhs.wrapping_neg();
        }

        let mut x = ((self.significand as u64) << -OFFSET) / rhs as u64;
        let mut overflowed = false;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            overflowed |= x != 0 && shift > x.leading_zeros();

            if shift >= u64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = (x & mask).wrapping_add(x >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add(temp);
            }
        }

        overflowed |= (negative && x != 0) || x > u32::MAX as u64;

        (
            Self {
                significand: x as u32,
            },
            overflowed,
        )
    }

    /// Computes `self / rhs`, wrapping around at the numeric bounds of the type.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn wrapping_div_signed<const R: i32>(self, rhs: I32F<R>) -> Self {
        self.overflowing_div_signed(rhs).0
    }

    /// Computes `self / rhs`, saturating at the numeric bounds of the type instead of overflowing.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn saturating_div_signed<const R: i32>(self, rhs: I32F<R>) -> Self {
        match self.overflowing_div_signed(rhs) {
            (_, true) => {
                if rhs.significand.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
            (x, _) => x,
        }
    }
}

impl U32F<-31> {
    /// Computes `cos(π * self)` using a minimax second-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.60096 ⋅ 10<sup>-2</sup>.
    #[must_use]
    pub const fn cospi_2(self) -> I32F<-30> {
        let theta = self.significand as i32;
        let significand = crate::algorithm::cospi_i32_2(theta);

        I32F { significand }
    }

    /// Computes `cos(π * self)` using a minimax fourth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.18799 ⋅ 10<sup>-4</sup>.
    #[must_use]
    pub const fn cospi_4(self) -> I32F<-30> {
        let theta = self.significand as i32;
        let significand = crate::algorithm::cospi_i32_4(theta);

        I32F { significand }
    }

    /// Computes `cos(π * self)` using a minimax sixth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.20285 ⋅ 10<sup>-6</sup>.
    #[must_use]
    pub const fn cospi_6(self) -> I32F<-30> {
        let theta = self.significand as i32;
        let significand = crate::algorithm::cospi_i32_6(theta);

        I32F { significand }
    }

    /// Computes `cos(π * self)` using a minimax eighth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.98045 ⋅ 10<sup>-8</sup>.
    #[must_use]
    pub const fn cospi_8(self) -> I32F<-30> {
        let theta = self.significand as i32;
        let significand = crate::algorithm::cospi_i32_8(theta);

        I32F { significand }
    }

    /// Computes `cos(π * self)` using a minimax tenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 2.70068 ⋅ 10<sup>-10</sup>.
    #[must_use]
    pub const fn cospi_10(self) -> I32F<-30> {
        let theta = self.significand as i32;
        let significand = crate::algorithm::cospi_i32_10(theta);

        I32F { significand }
    }

    /// Computes `sin(π * self)` using a minimax second-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.60096 ⋅ 10<sup>-2</sup>.
    #[must_use]
    pub const fn sinpi_2(self) -> I32F<-30> {
        const PHASE_SHIFT: i32 = 0x3 << (i32::BITS - 2);

        let theta = (self.significand as i32).wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i32_2(theta);

        I32F { significand }
    }

    /// Computes `sin(π * self)` using a minimax fourth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.18799 ⋅ 10<sup>-4</sup>.
    #[must_use]
    pub const fn sinpi_4(self) -> I32F<-30> {
        const PHASE_SHIFT: i32 = 0x3 << (i32::BITS - 2);

        let theta = (self.significand as i32).wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i32_4(theta);

        I32F { significand }
    }

    /// Computes `sin(π * self)` using a minimax sixth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.20285 ⋅ 10<sup>-6</sup>.
    #[must_use]
    pub const fn sinpi_6(self) -> I32F<-30> {
        const PHASE_SHIFT: i32 = 0x3 << (i32::BITS - 2);

        let theta = (self.significand as i32).wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i32_6(theta);

        I32F { significand }
    }

    /// Computes `sin(π * self)` using a minimax eighth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.98045 ⋅ 10<sup>-8</sup>.
    #[must_use]
    pub const fn sinpi_8(self) -> I32F<-30> {
        const PHASE_SHIFT: i32 = 0x3 << (i32::BITS - 2);

        let theta = (self.significand as i32).wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i32_8(theta);

        I32F { significand }
    }

    /// Computes `sin(π * self)` using a minimax tenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 2.70068 ⋅ 10<sup>-10</sup>.
    #[must_use]
    pub const fn sinpi_10(self) -> I32F<-30> {
        const PHASE_SHIFT: i32 = 0x3 << (i32::BITS - 2);

        let theta = (self.significand as i32).wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i32_10(theta);

        I32F { significand }
    }
}

impl From<U32F<0>> for u32 {
    fn from(value: U32F<0>) -> Self {
        value.significand
    }
}

impl From<u32> for U32F<0> {
    fn from(value: u32) -> Self {
        Self { significand: value }
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
        self.partial_cmp(other) == Some(cmp::Ordering::Equal)
    }
}

impl<const E1: i32, const E2: i32> PartialOrd<U32F<E2>> for U32F<E1> {
    fn partial_cmp(&self, other: &U32F<E2>) -> Option<cmp::Ordering> {
        let mut lhs = self.significand;
        let mut rhs = other.significand;

        if E1 > E2 {
            let shift = E1.wrapping_sub(E2).cast_unsigned();
            let temp = lhs.unbounded_shl(shift);

            if temp.unbounded_shr(shift) != lhs {
                return Some(cmp::Ordering::Greater);
            }

            lhs = temp;
        } else if E2 > E1 {
            let shift = E2.wrapping_sub(E1).cast_unsigned();
            let temp = rhs.unbounded_shl(shift);

            if temp.unbounded_shr(shift) != rhs {
                return Some(cmp::Ordering::Less);
            }

            rhs = temp;
        }

        PartialOrd::partial_cmp(&lhs, &rhs)
    }
}

impl<const E: i32> fmt::Debug for U32F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U32F<{E}")?;

        f.debug_tuple(">").field(&self.significand).finish()
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

impl<const E: i32> ops::Add for U32F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self::add(self, rhs)
    }
}

impl<const E: i32> ops::Add<I32F<E>> for U32F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: I32F<E>) -> Self::Output {
        Self::add_signed(self, rhs)
    }
}

impl<const E: i32> ops::Sub for U32F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::sub(self, rhs)
    }
}

impl<const E: i32> ops::Sub<I32F<E>> for U32F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: I32F<E>) -> Self::Output {
        Self::sub_signed(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Mul<U32F<R>> for U32F<E> {
    type Output = Self;

    #[track_caller]
    fn mul(self, rhs: U32F<R>) -> Self::Output {
        Self::mul(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Mul<I32F<R>> for U32F<E> {
    type Output = Self;

    #[track_caller]
    fn mul(self, rhs: I32F<R>) -> Self::Output {
        Self::mul_signed(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Div<U32F<R>> for U32F<E> {
    type Output = Self;

    #[track_caller]
    fn div(self, rhs: U32F<R>) -> Self::Output {
        Self::div(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Div<I32F<R>> for U32F<E> {
    type Output = Self;

    #[track_caller]
    fn div(self, rhs: I32F<R>) -> Self::Output {
        Self::div_signed(self, rhs)
    }
}

impl<const E: i32> ops::AddAssign for U32F<E> {
    #[track_caller]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(*self, rhs)
    }
}

impl<const E: i32> ops::AddAssign<I32F<E>> for U32F<E> {
    #[track_caller]
    fn add_assign(&mut self, rhs: I32F<E>) {
        *self = Self::add_signed(*self, rhs)
    }
}

impl<const E: i32> ops::SubAssign for U32F<E> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::sub(*self, rhs)
    }
}

impl<const E: i32> ops::SubAssign<I32F<E>> for U32F<E> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: I32F<E>) {
        *self = Self::sub_signed(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::MulAssign<U32F<R>> for U32F<E> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: U32F<R>) {
        *self = Self::mul(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::MulAssign<I32F<R>> for U32F<E> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: I32F<R>) {
        *self = Self::mul_signed(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::DivAssign<U32F<R>> for U32F<E> {
    #[track_caller]
    fn div_assign(&mut self, rhs: U32F<R>) {
        *self = Self::div(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::DivAssign<I32F<R>> for U32F<E> {
    #[track_caller]
    fn div_assign(&mut self, rhs: I32F<R>) {
        *self = Self::div_signed(*self, rhs)
    }
}
