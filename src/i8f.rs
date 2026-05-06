use ::core::cmp;
use ::core::fmt;
use ::core::ops;

use crate::I16F;
use crate::I32F;
use crate::I64F;
use crate::I128F;
use crate::U8F;
use crate::U16F;
use crate::U32F;
use crate::U64F;
use crate::U128F;
use crate::error::TryFromFloatError;

/// The 32-bit unsigned fixed-point type.
#[derive(Clone, Copy, Eq, Ord)]
pub struct I8F<const E: i32> {
    pub(crate) significand: i8,
}

impl I8F<-9> {
    /// 1/τ
    pub const FRAC_1_TAU: Self = Self { significand: 0x51 };
}

impl I8F<-8> {
    /// 1/π
    pub const FRAC_1_PI: Self = Self { significand: 0x51 };
    /// π/8
    pub const FRAC_PI_8: Self = Self { significand: 0x65 };
    /// log<sub>10</sub>(2)
    pub const LOG10_2: Self = Self { significand: 0x4D };
    /// log<sub>10</sub>(e)
    pub const LOG10_E: Self = Self { significand: 0x6F };
}

impl I8F<-7> {
    /// The Euler-Mascheroni constant (γ)
    pub const EULER_GAMMA: Self = Self { significand: 0x4A };
    /// 1/sqrt(2)
    pub const FRAC_1_SQRT_2: Self = Self { significand: 0x5B };
    /// 2/π
    pub const FRAC_2_PI: Self = Self { significand: 0x51 };
    /// π/4
    pub const FRAC_PI_4: Self = Self { significand: 0x65 };
    /// π/6
    pub const FRAC_PI_6: Self = Self { significand: 0x43 };
    /// ln(2)
    pub const LN_2: Self = Self { significand: 0x59 };
}

impl I8F<-6> {
    /// 2/sqrt(π)
    pub const FRAC_2_SQRT_PI: Self = Self { significand: 0x48 };
    /// π/2
    pub const FRAC_PI_2: Self = Self { significand: 0x65 };
    /// π/3
    pub const FRAC_PI_3: Self = Self { significand: 0x43 };
    /// The golden ratio (φ)
    pub const GOLDEN_RATIO: Self = Self { significand: 0x68 };
    /// log<sub>2</sub>(e)
    pub const LOG2_E: Self = Self { significand: 0x5C };
    /// sqrt(2)
    pub const SQRT_2: Self = Self { significand: 0x5B };
}

impl I8F<-5> {
    /// Euler's number (e)
    pub const E: Self = Self { significand: 0x57 };
    /// ln(10)
    pub const LN_10: Self = Self { significand: 0x4A };
    /// log<sub>2</sub>(10)
    pub const LOG2_10: Self = Self { significand: 0x6A };
    /// Archimedes’ constant (π)
    pub const PI: Self = Self { significand: 0x65 };
}

impl I8F<-4> {
    /// The full circle constant (τ)
    pub const TAU: Self = Self { significand: 0x65 };
}

impl<const E: i32> I8F<E> {
    /// The smallest value that can be represented by this fixed-point type, equal to -2<sup>7</sup> ⋅ 2<sup>E</sup>.
    pub const MIN: Self = Self {
        significand: i8::MIN,
    };

    /// The largest value that can be represented by this fixed-point type, equal to (2<sup>7</sup> - 1) ⋅ 2<sup>E</sup>.
    pub const MAX: Self = Self {
        significand: i8::MAX,
    };

    /// The size of this type in bits.
    pub const BITS: u32 = i8::BITS;

    /// Creates a new fixed-point number from an integer significand, equal to `significand` ⋅ 2<sup>E</sup>.
    #[must_use]
    pub const fn new(significand: i8) -> Self {
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

        let mut significand = significand.cast_signed();

        if sign != 0 {
            significand = significand.wrapping_neg();
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

        if significand < i8::MIN as i32 || significand > i8::MAX as i32 {
            return Err(TryFromFloatError::Overflow);
        }

        let significand = significand as i8;

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

        let mut significand = significand.cast_signed();

        if sign != 0 {
            significand = significand.wrapping_neg();
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

        if significand < i8::MIN as i64 || significand > i8::MAX as i64 {
            return Err(TryFromFloatError::Overflow);
        }

        let significand = significand as i8;

        Ok(Self { significand })
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

    /// Converts from [`I16F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_i16f(value: I16F<E>) -> (Self, bool) {
        let overflowed = value.significand < i8::MIN as i16 || value.significand > i8::MAX as i16;
        let significand = value.significand as i8;

        (Self { significand }, overflowed)
    }

    /// Converts from [`I16F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_i16f(value: I16F<E>) -> Self {
        Self::overflowing_from_i16f(value).0
    }

    /// Converts from [`I16F`], saturating at the numeric bounds of the type instead of overflowing.
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
        let overflowed = value.significand < i8::MIN as i32 || value.significand > i8::MAX as i32;
        let significand = value.significand as i8;

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
        let overflowed = value.significand < i8::MIN as i64 || value.significand > i8::MAX as i64;
        let significand = value.significand as i8;

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
        let overflowed = value.significand < i8::MIN as i128 || value.significand > i8::MAX as i128;
        let significand = value.significand as i8;

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

    /// Converts from [`U8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_u8f(value: U8F<E>) -> Self {
        match Self::overflowing_from_u8f(value) {
            (_, true) if cfg!(debug_assertions) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`U8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_u8f(value: U8F<E>) -> Self {
        match Self::overflowing_from_u8f(value) {
            (_, true) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`U8F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_u8f(value: U8F<E>) -> Option<Self> {
        match Self::overflowing_from_u8f(value) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Converts from [`U8F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_u8f(value: U8F<E>) -> (Self, bool) {
        let overflowed = value.significand > i8::MAX as u8;
        let significand = value.significand as i8;

        (Self { significand }, overflowed)
    }

    /// Converts from [`U8F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_u8f(value: U8F<E>) -> Self {
        Self::overflowing_from_u8f(value).0
    }

    /// Converts from [`U8F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_u8f(value: U8F<E>) -> Self {
        match Self::overflowing_from_u8f(value) {
            (_, true) => Self::MAX,
            (x, _) => x,
        }
    }

    /// Converts from [`U16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_u16f(value: U16F<E>) -> Self {
        match Self::overflowing_from_u16f(value) {
            (_, true) if cfg!(debug_assertions) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`U16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_u16f(value: U16F<E>) -> Self {
        match Self::overflowing_from_u16f(value) {
            (_, true) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`U16F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_u16f(value: U16F<E>) -> Option<Self> {
        match Self::overflowing_from_u16f(value) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Converts from [`U16F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_u16f(value: U16F<E>) -> (Self, bool) {
        let overflowed = value.significand > i8::MAX as u16;
        let significand = value.significand as i8;

        (Self { significand }, overflowed)
    }

    /// Converts from [`U16F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_u16f(value: U16F<E>) -> Self {
        Self::overflowing_from_u16f(value).0
    }

    /// Converts from [`U16F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_u16f(value: U16F<E>) -> Self {
        match Self::overflowing_from_u16f(value) {
            (_, true) => Self::MAX,
            (x, _) => x,
        }
    }

    /// Converts from [`U32F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn from_u32f(value: U32F<E>) -> Self {
        match Self::overflowing_from_u32f(value) {
            (_, true) if cfg!(debug_assertions) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`U32F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_from_u32f(value: U32F<E>) -> Self {
        match Self::overflowing_from_u32f(value) {
            (_, true) => crate::panic::from(),
            (x, _) => x,
        }
    }

    /// Converts from [`U32F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_from_u32f(value: U32F<E>) -> Option<Self> {
        match Self::overflowing_from_u32f(value) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Converts from [`U32F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_from_u32f(value: U32F<E>) -> (Self, bool) {
        let overflowed = value.significand > i8::MAX as u32;
        let significand = value.significand as i8;

        (Self { significand }, overflowed)
    }

    /// Converts from [`U32F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_from_u32f(value: U32F<E>) -> Self {
        Self::overflowing_from_u32f(value).0
    }

    /// Converts from [`U32F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_from_u32f(value: U32F<E>) -> Self {
        match Self::overflowing_from_u32f(value) {
            (_, true) => Self::MAX,
            (x, _) => x,
        }
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
        let overflowed = value.significand > i8::MAX as u64;
        let significand = value.significand as i8;

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
        let overflowed = value.significand > i8::MAX as u128;
        let significand = value.significand as i8;

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

    /// Raw transutation from [`u8`].
    #[must_use]
    pub const fn from_bits(bits: u8) -> Self {
        Self {
            significand: bits as i8,
        }
    }

    /// Creates a native endian fixed-point number from its memory representation as a byte array in native endian byte order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes), as appropriate, instead.
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; 1]) -> Self {
        Self {
            significand: i8::from_ne_bytes(bytes),
        }
    }

    /// Creates a fixed-point number from its memory representation as a byte array in big endian byte order.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 1]) -> Self {
        Self {
            significand: i8::from_be_bytes(bytes),
        }
    }

    /// Creates a fixed-point number from its memory representation as a byte array in little endian byte order.
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; 1]) -> Self {
        Self {
            significand: i8::from_le_bytes(bytes),
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
            let mut significand = self.significand as u32;

            if self.significand < 0 {
                bits |= 0x80000000;
                significand = significand.wrapping_neg();
            }

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

    /// Converts into [`I16F`] losslessly.
    #[must_use]
    pub const fn into_i16f(self) -> I16F<E> {
        I16F::from_i8f(self)
    }

    /// Converts into [`I32F`] losslessly.
    #[must_use]
    pub const fn into_i32f(self) -> I32F<E> {
        I32F::from_i8f(self)
    }

    /// Converts into [`I64F`] losslessly.
    #[must_use]
    pub const fn into_i64f(self) -> I64F<E> {
        I64F::from_i8f(self)
    }

    /// Converts into [`I128F`] losslessly.
    #[must_use]
    pub const fn into_i128f(self) -> I128F<E> {
        I128F::from_i8f(self)
    }

    /// Converts into [`U8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn into_u8f(self) -> U8F<E> {
        U8F::from_i8f(self)
    }

    /// Converts into [`U8F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u8f(self) -> U8F<E> {
        U8F::strict_from_i8f(self)
    }

    /// Converts into [`U8F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_into_u8f(self) -> Option<U8F<E>> {
        U8F::checked_from_i8f(self)
    }

    /// Converts into [`U8F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_into_u8f(self) -> (U8F<E>, bool) {
        U8F::overflowing_from_i8f(self)
    }

    /// Converts into [`U8F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_into_u8f(self) -> U8F<E> {
        U8F::wrapping_from_i8f(self)
    }

    /// Converts into [`U8F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_into_u8f(self) -> U8F<E> {
        U8F::saturating_from_i8f(self)
    }

    /// Converts into [`U16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn into_u16f(self) -> U16F<E> {
        U16F::from_i8f(self)
    }

    /// Converts into [`U16F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u16f(self) -> U16F<E> {
        U16F::strict_from_i8f(self)
    }

    /// Converts into [`U16F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_into_u16f(self) -> Option<U16F<E>> {
        U16F::checked_from_i8f(self)
    }

    /// Converts into [`U16F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_into_u16f(self) -> (U16F<E>, bool) {
        U16F::overflowing_from_i8f(self)
    }

    /// Converts into [`U16F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_into_u16f(self) -> U16F<E> {
        U16F::wrapping_from_i8f(self)
    }

    /// Converts into [`U16F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_into_u16f(self) -> U16F<E> {
        U16F::saturating_from_i8f(self)
    }

    /// Converts into [`U32F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn into_u32f(self) -> U32F<E> {
        U32F::from_i8f(self)
    }

    /// Converts into [`U32F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u32f(self) -> U32F<E> {
        U32F::strict_from_i8f(self)
    }

    /// Converts into [`U32F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_into_u32f(self) -> Option<U32F<E>> {
        U32F::checked_from_i8f(self)
    }

    /// Converts into [`U32F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_into_u32f(self) -> (U32F<E>, bool) {
        U32F::overflowing_from_i8f(self)
    }

    /// Converts into [`U32F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_into_u32f(self) -> U32F<E> {
        U32F::wrapping_from_i8f(self)
    }

    /// Converts into [`U32F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_into_u32f(self) -> U32F<E> {
        U32F::saturating_from_i8f(self)
    }

    /// Converts into [`U64F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn into_u64f(self) -> U64F<E> {
        U64F::from_i8f(self)
    }

    /// Converts into [`U64F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u64f(self) -> U64F<E> {
        U64F::strict_from_i8f(self)
    }

    /// Converts into [`U64F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_into_u64f(self) -> Option<U64F<E>> {
        U64F::checked_from_i8f(self)
    }

    /// Converts into [`U64F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_into_u64f(self) -> (U64F<E>, bool) {
        U64F::overflowing_from_i8f(self)
    }

    /// Converts into [`U64F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_into_u64f(self) -> U64F<E> {
        U64F::wrapping_from_i8f(self)
    }

    /// Converts into [`U64F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_into_u64f(self) -> U64F<E> {
        U64F::saturating_from_i8f(self)
    }

    /// Converts into [`U128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn into_u128f(self) -> U128F<E> {
        U128F::from_i8f(self)
    }

    /// Converts into [`U128F`], panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_into_u128f(self) -> U128F<E> {
        U128F::strict_from_i8f(self)
    }

    /// Converts into [`U128F`], returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_into_u128f(self) -> Option<U128F<E>> {
        U128F::checked_from_i8f(self)
    }

    /// Converts into [`U128F`]. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_into_u128f(self) -> (U128F<E>, bool) {
        U128F::overflowing_from_i8f(self)
    }

    /// Converts into [`U128F`], wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_into_u128f(self) -> U128F<E> {
        U128F::wrapping_from_i8f(self)
    }

    /// Converts into [`U128F`], saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_into_u128f(self) -> U128F<E> {
        U128F::saturating_from_i8f(self)
    }

    /// Raw transmutation to [`u8`].
    #[must_use]
    pub const fn to_bits(self) -> u8 {
        self.significand as u8
    }

    /// Returns the memory representation of this fixed-point number as a byte array in native byte order.
    #[must_use]
    pub const fn to_ne_bytes(self) -> [u8; 1] {
        self.significand.to_ne_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in big-endian (network) byte order.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 1] {
        self.significand.to_be_bytes()
    }

    /// Returns the memory representation of this fixed-point number as a byte array in little-endian byte order.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 1] {
        self.significand.to_le_bytes()
    }

    /// Returns the fixed-point significand, equal to `self` ⋅ 2<sup>-E</sup>.
    #[must_use]
    pub const fn significand(self) -> i8 {
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
    pub const fn rescale<const E2: i32>(self) -> I8F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::rescale();
            }

            if shift >= i8::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i8::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u8;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I8F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_rescale<const E2: i32>(self) -> I8F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::rescale();
            }

            if shift >= i8::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i8::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u8;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I8F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_rescale<const E2: i32>(self) -> I8F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if shift >= i8::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i8::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u8;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I8F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_rescale<const E2: i32>(self) -> I8F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                if x < 0 {
                    return I8F::MIN;
                } else {
                    return I8F::MAX;
                }
            }

            if shift >= i8::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i8::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u8;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I8F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_rescale<const E2: i32>(self) -> (I8F<E2>, bool) {
        let mut x = self.significand;
        let mut overflowed = false;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            overflowed |= x != 0 && shift >= x.leading_zeros() | x.leading_ones();

            if shift >= i8::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i8::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u8;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        (I8F { significand: x }, overflowed)
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_rescale<const E2: i32>(self) -> Option<I8F<E2>> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                return None;
            }

            if shift >= i8::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i8::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u8).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u8;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        Some(I8F { significand: x })
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
    pub const fn add_unsigned(self, rhs: U8F<E>) -> Self {
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
    pub const fn strict_add_unsigned(self, rhs: U8F<E>) -> Self {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        if x < self.significand {
            crate::panic::add();
        }

        Self { significand: x }
    }

    /// Computes `self + rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_add_unsigned(self, rhs: U8F<E>) -> Self {
        Self {
            significand: self.significand.wrapping_add_unsigned(rhs.significand),
        }
    }

    /// Computes `self + rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_add_unsigned(self, rhs: U8F<E>) -> Self {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        if x < self.significand {
            return Self::MAX;
        }

        Self { significand: x }
    }

    /// Computes `self + rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_add_unsigned(self, rhs: U8F<E>) -> (Self, bool) {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        (Self { significand: x }, x < self.significand)
    }

    /// Computes `self + rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_add_unsigned(self, rhs: U8F<E>) -> Option<Self> {
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
    pub const fn sub_unsigned(self, rhs: U8F<E>) -> Self {
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
    pub const fn strict_sub_unsigned(self, rhs: U8F<E>) -> Self {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        if x > self.significand {
            crate::panic::sub();
        }

        Self { significand: x }
    }

    /// Computes `self - rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_sub_unsigned(self, rhs: U8F<E>) -> Self {
        Self {
            significand: self.significand.wrapping_sub_unsigned(rhs.significand),
        }
    }

    /// Computes `self - rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_sub_unsigned(self, rhs: U8F<E>) -> Self {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        if x > self.significand {
            return Self::MIN;
        }

        Self { significand: x }
    }

    /// Computes `self - rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_sub_unsigned(self, rhs: U8F<E>) -> (Self, bool) {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        (Self { significand: x }, x > self.significand)
    }

    /// Computes `self - rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_sub_unsigned(self, rhs: U8F<E>) -> Option<Self> {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        if x > self.significand {
            return None;
        }

        Some(Self { significand: x })
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn mul<const R: i32>(self, rhs: I8F<R>) -> Self {
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
    pub const fn strict_mul<const R: i32>(self, rhs: I8F<R>) -> Self {
        match self.overflowing_mul(rhs) {
            (_, true) => crate::panic::mul(),
            (x, _) => x,
        }
    }

    /// Computes `self * rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_mul<const R: i32>(self, rhs: I8F<R>) -> Option<Self> {
        match self.overflowing_mul(rhs) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Computes `self * rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_mul<const R: i32>(self, rhs: I8F<R>) -> (Self, bool) {
        let mut significand = self.significand as i16 * rhs.significand as i16;
        let mut overflowed = false;

        if R >= 0 {
            let shift = R.cast_unsigned();
            let temp = significand.unbounded_shl(shift);
            overflowed |= temp.unbounded_shr(shift) != significand;
            significand = temp;
        } else {
            let shift = R.wrapping_neg().cast_unsigned();

            if shift >= i16::BITS {
                significand = 0;
            } else {
                significand += significand >> shift & 0x1;
                significand += !(!0 << (shift - 1));
                significand >>= shift;
            }
        }

        overflowed |= significand < i8::MIN as i16 || significand > i8::MAX as i16;
        let significand = significand as i8;

        (Self { significand }, overflowed)
    }

    /// Computes `self * rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_mul<const R: i32>(self, rhs: I8F<R>) -> Self {
        self.overflowing_mul(rhs).0
    }

    /// Computes `self * rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_mul<const R: i32>(self, rhs: I8F<R>) -> Self {
        match self.overflowing_mul(rhs) {
            (_, true) => {
                if self.significand.is_negative() != rhs.significand.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
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
    pub const fn mul_unsigned<const R: i32>(self, rhs: U8F<R>) -> Self {
        match self.overflowing_mul_unsigned(rhs) {
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
    pub const fn strict_mul_unsigned<const R: i32>(self, rhs: U8F<R>) -> Self {
        match self.overflowing_mul_unsigned(rhs) {
            (_, true) => crate::panic::mul(),
            (x, _) => x,
        }
    }

    /// Computes `self * rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_mul_unsigned<const R: i32>(self, rhs: U8F<R>) -> Option<Self> {
        match self.overflowing_mul_unsigned(rhs) {
            (_, true) => None,
            (x, _) => Some(x),
        }
    }

    /// Computes `self * rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_mul_unsigned<const R: i32>(self, rhs: U8F<R>) -> (Self, bool) {
        let mut significand = self.significand as i16 * rhs.significand as i16;
        let mut overflowed = false;

        if R >= 0 {
            let shift = R.cast_unsigned();
            let temp = significand.unbounded_shl(shift);
            overflowed |= temp.unbounded_shr(shift) != significand;
            significand = temp;
        } else {
            let shift = R.wrapping_neg().cast_unsigned();

            if shift >= i16::BITS {
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

        overflowed |= significand < i8::MIN as i16 || significand > i8::MAX as i16;
        let significand = significand as i8;

        (Self { significand }, overflowed)
    }

    /// Computes `self * rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_mul_unsigned<const R: i32>(self, rhs: U8F<R>) -> Self {
        self.overflowing_mul_unsigned(rhs).0
    }

    /// Computes `self * rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_mul_unsigned<const R: i32>(self, rhs: U8F<R>) -> Self {
        match self.overflowing_mul_unsigned(rhs) {
            (_, true) => {
                if self.significand.is_negative() {
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
    pub const fn div<const R: i32>(self, rhs: I8F<R>) -> Self {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i16) << -OFFSET).overflowing_div(rhs.significand as i16);
        let negative = (x < 0) != overflowed;
        let mut x = x as u16;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift > x.leading_zeros() {
                crate::panic::div();
            }

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if cfg!(debug_assertions) && x > i8::MAX as u16 + negative as u16 {
            crate::panic::div();
        }

        if negative {
            x = x.wrapping_neg();
        }

        Self {
            significand: x as i8,
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
    pub const fn strict_div<const R: i32>(self, rhs: I8F<R>) -> Self {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i16) << -OFFSET).overflowing_div(rhs.significand as i16);
        let negative = (x < 0) != overflowed;
        let mut x = x as u16;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() {
                crate::panic::div();
            }

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x > (i8::MAX as u16).wrapping_add(negative as u16) {
            crate::panic::div();
        }

        if negative {
            x = x.wrapping_neg();
        }

        Self {
            significand: x as i8,
        }
    }

    /// Computes `self / rhs`, wrapping around at the numeric bounds of the type.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn wrapping_div<const R: i32>(self, rhs: I8F<R>) -> Self {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i16) << -OFFSET).overflowing_div(rhs.significand as i16);
        let negative = (x < 0) != overflowed;
        let mut x = x as u16;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u16).unbounded_shl(shift.wrapping_sub(1))
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
            significand: x as i8,
        }
    }

    /// Computes `self / rhs`, saturating at the numeric bounds of the type instead of overflowing.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn saturating_div<const R: i32>(self, rhs: I8F<R>) -> Self {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i16) << -OFFSET).overflowing_div(rhs.significand as i16);
        let negative = (x < 0) != overflowed;
        let mut x = x as u16;

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

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x > (i8::MAX as u16).wrapping_add(negative as u16) {
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
            significand: x as i8,
        }
    }

    /// Computes `self / rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn overflowing_div<const R: i32>(self, rhs: I8F<R>) -> (Self, bool) {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i16) << -OFFSET).overflowing_div(rhs.significand as i16);
        let negative = (x < 0) != overflowed;
        let mut x = x as u16;
        let mut overflowed = false;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            overflowed |= x != 0 && shift > x.leading_zeros();

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        overflowed |= x > (i8::MAX as u16).wrapping_add(negative as u16);

        if negative {
            x = x.wrapping_neg();
        }

        (
            Self {
                significand: x as i8,
            },
            overflowed,
        )
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or overflow occurred.
    #[must_use]
    pub const fn checked_div<const R: i32>(self, rhs: I8F<R>) -> Option<Self> {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        if rhs.significand == 0 {
            return None;
        }

        let (x, overflowed) =
            ((self.significand as i16) << -OFFSET).overflowing_div(rhs.significand as i16);
        let negative = (x < 0) != overflowed;
        let mut x = x as u16;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() {
                return None;
            }

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x > (i8::MAX as u16).wrapping_add(negative as u16) {
            return None;
        }

        if negative {
            x = x.wrapping_neg();
        }

        Some(Self {
            significand: x as i8,
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
    pub const fn div_unsigned<const R: i32>(self, rhs: U8F<R>) -> Self {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        let mut x = ((self.significand as i16) << -OFFSET) / rhs.significand as i16;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                crate::panic::div();
            }

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if cfg!(debug_assertions) && (x < i8::MIN as i16 || x > i8::MAX as i16) {
            crate::panic::div();
        }

        Self {
            significand: x as i8,
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
    pub const fn strict_div_unsigned<const R: i32>(self, rhs: U8F<R>) -> Self {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        let mut x = ((self.significand as i16) << -OFFSET) / rhs.significand as i16;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                crate::panic::div();
            }

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i8::MIN as i16 || x > i8::MAX as i16 {
            crate::panic::div();
        }

        Self {
            significand: x as i8,
        }
    }

    /// Computes `self / rhs`, wrapping around at the numeric bounds of the type.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn wrapping_div_unsigned<const R: i32>(self, rhs: U8F<R>) -> Self {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        let mut x = ((self.significand as i16) << -OFFSET) / rhs.significand as i16;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        Self {
            significand: x as i8,
        }
    }

    /// Computes `self / rhs`, saturating at the numeric bounds of the type instead of overflowing.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn saturating_div_unsigned<const R: i32>(self, rhs: U8F<R>) -> Self {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        let mut x = ((self.significand as i16) << -OFFSET) / rhs.significand as i16;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                if x < 0 {
                    return Self::MIN;
                } else {
                    return Self::MAX;
                }
            }

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i8::MIN as i16 {
            return Self::MIN;
        } else if x > i8::MAX as i16 {
            return Self::MAX;
        }

        Self {
            significand: x as i8,
        }
    }

    /// Computes `self / rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn overflowing_div_unsigned<const R: i32>(self, rhs: U8F<R>) -> (Self, bool) {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        let mut x = ((self.significand as i16) << -OFFSET) / rhs.significand as i16;
        let mut overflowed = false;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            overflowed |= x != 0 && shift > x.leading_zeros() | x.leading_ones();

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        overflowed |= x < i8::MIN as i16 || x > i8::MAX as i16;

        (
            Self {
                significand: x as i8,
            },
            overflowed,
        )
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or overflow occurred.
    #[must_use]
    pub const fn checked_div_unsigned<const R: i32>(self, rhs: U8F<R>) -> Option<Self> {
        const OFFSET: i32 = i8::BITS.cast_signed() - i16::BITS.cast_signed();

        if rhs.significand == 0 {
            return None;
        }

        let mut x = ((self.significand as i16) << -OFFSET) / rhs.significand as i16;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                return None;
            }

            if shift >= u16::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R > OFFSET } {
            let shift = const { R.wrapping_sub(OFFSET).cast_unsigned() };

            if shift >= u16::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i16).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i8::MIN as i16 || x > i8::MAX as i16 {
            return None;
        }

        Some(Self {
            significand: x as i8,
        })
    }
}

impl I8F<-7> {
    /// Computes `cos(π * self)` using a minimax second-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.60096 ⋅ 10<sup>-2</sup>.
    #[must_use]
    pub const fn cospi_2(self) -> I8F<-6> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i8_2(theta);

        I8F { significand }
    }

    /// Computes `cos(π * self)` using a minimax fourth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.18799 ⋅ 10<sup>-4</sup>.
    #[must_use]
    pub const fn cospi_4(self) -> I8F<-6> {
        let theta = self.significand;
        let significand = crate::algorithm::cospi_i8_4(theta);

        I8F { significand }
    }

    /// Computes `sin(π * self)` using a minimax second-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.60096 ⋅ 10<sup>-2</sup>.
    #[must_use]
    pub const fn sinpi_2(self) -> I8F<-6> {
        const PHASE_SHIFT: i8 = 0x3 << (i8::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i8_2(theta);

        I8F { significand }
    }

    /// Computes `sin(π * self)` using a minimax fourth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.18799 ⋅ 10<sup>-4</sup>.
    #[must_use]
    pub const fn sinpi_4(self) -> I8F<-6> {
        const PHASE_SHIFT: i8 = 0x3 << (i8::BITS - 2);

        let theta = self.significand.wrapping_add(PHASE_SHIFT);
        let significand = crate::algorithm::cospi_i8_4(theta);

        I8F { significand }
    }
}

impl From<I8F<0>> for i8 {
    fn from(value: I8F<0>) -> Self {
        value.significand
    }
}

impl From<i8> for I8F<0> {
    fn from(value: i8) -> Self {
        Self { significand: value }
    }
}

impl<const E1: i32, const E2: i32> PartialEq<I8F<E2>> for I8F<E1> {
    fn eq(&self, other: &I8F<E2>) -> bool {
        self.partial_cmp(other) == Some(cmp::Ordering::Equal)
    }
}

impl<const E1: i32, const E2: i32> PartialOrd<I8F<E2>> for I8F<E1> {
    fn partial_cmp(&self, other: &I8F<E2>) -> Option<cmp::Ordering> {
        let mut lhs = self.significand;
        let mut rhs = other.significand;

        if E1 > E2 {
            let shift = E1.wrapping_sub(E2).cast_unsigned();
            let temp = lhs.unbounded_shl(shift);

            if temp.unbounded_shr(shift) != lhs {
                if lhs.is_negative() {
                    return Some(cmp::Ordering::Less);
                } else {
                    return Some(cmp::Ordering::Greater);
                }
            }

            lhs = temp;
        } else if E2 > E1 {
            let shift = E2.wrapping_sub(E1).cast_unsigned();
            let temp = rhs.unbounded_shl(shift);

            if temp.unbounded_shr(shift) != rhs {
                if rhs.is_negative() {
                    return Some(cmp::Ordering::Greater);
                } else {
                    return Some(cmp::Ordering::Less);
                }
            }

            rhs = temp;
        }

        PartialOrd::partial_cmp(&lhs, &rhs)
    }
}

impl<const E: i32> fmt::Debug for I8F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I8F<{E}")?;

        f.debug_tuple(">").field(&self.significand).finish()
    }
}

impl<const E: i32> fmt::Binary for I8F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::Octal for I8F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::LowerHex for I8F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> fmt::UpperHex for I8F<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.to_bits(), f)
    }
}

impl<const E: i32> ops::Neg for I8F<E> {
    type Output = Self;

    #[track_caller]
    fn neg(self) -> Self::Output {
        Self::neg(self)
    }
}

impl<const E: i32> ops::Add for I8F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self::add(self, rhs)
    }
}

impl<const E: i32> ops::Add<U8F<E>> for I8F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: U8F<E>) -> Self::Output {
        Self::add_unsigned(self, rhs)
    }
}

impl<const E: i32> ops::Sub for I8F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::sub(self, rhs)
    }
}

impl<const E: i32> ops::Sub<U8F<E>> for I8F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: U8F<E>) -> Self::Output {
        Self::sub_unsigned(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Mul<I8F<R>> for I8F<E> {
    type Output = Self;

    #[track_caller]
    fn mul(self, rhs: I8F<R>) -> Self::Output {
        Self::mul(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Mul<U8F<R>> for I8F<E> {
    type Output = Self;

    #[track_caller]
    fn mul(self, rhs: U8F<R>) -> Self::Output {
        Self::mul_unsigned(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Div<I8F<R>> for I8F<E> {
    type Output = Self;

    #[track_caller]
    fn div(self, rhs: I8F<R>) -> Self::Output {
        Self::div(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Div<U8F<R>> for I8F<E> {
    type Output = Self;

    #[track_caller]
    fn div(self, rhs: U8F<R>) -> Self::Output {
        Self::div_unsigned(self, rhs)
    }
}

impl<const E: i32> ops::AddAssign for I8F<E> {
    #[track_caller]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(*self, rhs)
    }
}

impl<const E: i32> ops::AddAssign<U8F<E>> for I8F<E> {
    #[track_caller]
    fn add_assign(&mut self, rhs: U8F<E>) {
        *self = Self::add_unsigned(*self, rhs)
    }
}

impl<const E: i32> ops::SubAssign for I8F<E> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::sub(*self, rhs)
    }
}

impl<const E: i32> ops::SubAssign<U8F<E>> for I8F<E> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: U8F<E>) {
        *self = Self::sub_unsigned(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::MulAssign<I8F<R>> for I8F<E> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: I8F<R>) {
        *self = Self::mul(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::MulAssign<U8F<R>> for I8F<E> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: U8F<R>) {
        *self = Self::mul_unsigned(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::DivAssign<I8F<R>> for I8F<E> {
    #[track_caller]
    fn div_assign(&mut self, rhs: I8F<R>) {
        *self = Self::div(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::DivAssign<U8F<R>> for I8F<E> {
    #[track_caller]
    fn div_assign(&mut self, rhs: U8F<R>) {
        *self = Self::div_unsigned(*self, rhs)
    }
}
