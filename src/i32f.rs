use ::core::cmp;
use ::core::fmt;
use ::core::ops;

use crate::I8F;
use crate::I16F;
use crate::U8F;
use crate::U16F;

use crate::U32F;
use crate::error::TryFromFloatError;

/// The 32-bit signed fixed-point type.
#[derive(Clone, Copy, Eq, Hash, Ord)]
pub struct I32F<const E: i32> {
    pub(crate) significand: i32,
}

impl I32F<-33> {
    /// 1/τ
    pub const FRAC_1_TAU: Self = Self {
        significand: 0x517CC1B7,
    };
}

impl I32F<-32> {
    /// 1/π
    pub const FRAC_1_PI: Self = Self {
        significand: 0x517CC1B7,
    };
    /// π/8
    pub const FRAC_PI_8: Self = Self {
        significand: 0x6487ED51,
    };
    /// log<sub>10</sub>(2)
    pub const LOG10_2: Self = Self {
        significand: 0x4D104D42,
    };
    /// log<sub>10</sub>(e)
    pub const LOG10_E: Self = Self {
        significand: 0x6F2DEC55,
    };
}

impl I32F<-31> {
    /// The Euler-Mascheroni constant (γ)
    pub const EULER_GAMMA: Self = Self {
        significand: 0x49E233F2,
    };
    /// 1/sqrt(2)
    pub const FRAC_1_SQRT_2: Self = Self {
        significand: 0x5A82799A,
    };
    /// 2/π
    pub const FRAC_2_PI: Self = Self {
        significand: 0x517CC1B7,
    };
    /// π/4
    pub const FRAC_PI_4: Self = Self {
        significand: 0x6487ED51,
    };
    /// π/6
    pub const FRAC_PI_6: Self = Self {
        significand: 0x430548E1,
    };
    /// ln(2)
    pub const LN_2: Self = Self {
        significand: 0x58B90BFC,
    };
}

impl I32F<-30> {
    /// 2/sqrt(π)
    pub const FRAC_2_SQRT_PI: Self = Self {
        significand: 0x48375D41,
    };
    /// π/2
    pub const FRAC_PI_2: Self = Self {
        significand: 0x6487ED51,
    };
    /// π/3
    pub const FRAC_PI_3: Self = Self {
        significand: 0x430548E1,
    };
    /// The golden ratio (φ)
    pub const GOLDEN_RATIO: Self = Self {
        significand: 0x678DDE6E,
    };
    /// log<sub>2</sub>(e)
    pub const LOG2_E: Self = Self {
        significand: 0x5C551D95,
    };
    /// sqrt(2)
    pub const SQRT_2: Self = Self {
        significand: 0x5A82799A,
    };
}

impl I32F<-29> {
    /// Euler's number (e)
    pub const E: Self = Self {
        significand: 0x56FC2A2C,
    };
    /// ln(10)
    pub const LN_10: Self = Self {
        significand: 0x49AEC6EF,
    };
    /// log<sub>2</sub>(10)
    pub const LOG2_10: Self = Self {
        significand: 0x6A4D3C26,
    };
    /// Archimedes’ constant (π)
    pub const PI: Self = Self {
        significand: 0x6487ED51,
    };
}

impl I32F<-28> {
    /// The full circle constant (τ)
    pub const TAU: Self = Self {
        significand: 0x6487ED51,
    };
}

impl<const E: i32> I32F<E> {
    /// The smallest value that can be represented by this fixed-point type, equal to -2<sup>31</sup> ⋅ 2<sup>E</sup>.
    pub const MIN: Self = Self {
        significand: i32::MIN,
    };

    /// The largest value that can be represented by this fixed-point type, equal to (2<sup>31</sup> - 1) ⋅ 2<sup>E</sup>.
    pub const MAX: Self = Self {
        significand: i32::MAX,
    };

    /// The size of this type in bits.
    pub const BITS: u32 = i32::BITS;

    /// Creates a new fixed-point number from an integer significand, equal to `significand` ⋅ 2<sup>E</sup>.
    #[inline(always)]
    #[must_use]
    pub const fn new(significand: i32) -> Self {
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

        let mut significand = significand as i32;

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

            if shift >= i32::BITS {
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

        if significand < i32::MIN as i64 {
            return Err(TryFromFloatError::Underflow);
        } else if significand > i32::MAX as i64 {
            return Err(TryFromFloatError::Overflow);
        }

        Ok(Self {
            significand: significand as i32,
        })
    }

    /// Converts from [`I8F<E>`] losslessly.
    #[must_use]
    pub const fn from_i8f(value: I8F<E>) -> Self {
        Self {
            significand: value.significand as i32,
        }
    }

    /// Converts from [`I16F<E>`] losslessly.
    #[must_use]
    pub const fn from_i16f(value: I16F<E>) -> Self {
        Self {
            significand: value.significand as i32,
        }
    }

    /// Converts from [`U8F<E>`] losslessly.
    #[must_use]
    pub const fn from_u8f(value: U8F<E>) -> Self {
        Self {
            significand: value.significand as i32,
        }
    }

    /// Converts from [`U16F<E>`] losslessly.
    #[must_use]
    pub const fn from_u16f(value: U16F<E>) -> Self {
        Self {
            significand: value.significand as i32,
        }
    }

    /// Raw transutation from [`u32`].
    #[inline(always)]
    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        Self {
            significand: bits.cast_signed(),
        }
    }

    /// Creates a native endian fixed-point number from its memory representation as a byte array in native endian byte order.
    ///
    /// As the target platform's native endianness is used, portable code likely wants to use [`from_be_bytes`](Self::from_be_bytes) or [`from_le_bytes`](Self::from_le_bytes), as appropriate, instead.
    #[must_use]
    pub const fn from_ne_bytes(bytes: [u8; 4]) -> Self {
        Self {
            significand: i32::from_ne_bytes(bytes),
        }
    }

    /// Creates a fixed-point number from its memory representation as a byte array in big endian byte order.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self {
            significand: i32::from_be_bytes(bytes),
        }
    }

    /// Creates a fixed-point number from its memory representation as a byte array in little endian byte order.
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self {
            significand: i32::from_le_bytes(bytes),
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

    /// Raw transmutation to [`u32`].
    #[inline(always)]
    #[must_use]
    pub const fn to_bits(self) -> u32 {
        self.significand.cast_unsigned()
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
    #[inline(always)]
    #[must_use]
    pub const fn significand(self) -> i32 {
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
    /// This function will panic if `self` is zero or negative, or if overflow occurred.
    #[must_use]
    #[track_caller]
    pub const fn ilog2(self) -> i32 {
        let x = self.significand.ilog2();
        let Some(x) = E.checked_add_unsigned(x) else {
            crate::panic::ilog2();
        };

        x
    }

    /// Computes the base 2 logarithm of `self`, rounded down. Returns `None` if `self` is zero or negative, or if overflow occurred.
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
    pub const fn rescale<const E2: i32>(self) -> I32F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::rescale();
            }

            if shift >= i32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i32::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u32;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I32F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_rescale<const E2: i32>(self) -> I32F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::rescale();
            }

            if shift >= i32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i32::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u32;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I32F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_rescale<const E2: i32>(self) -> I32F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if shift >= i32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i32::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u32;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I32F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_rescale<const E2: i32>(self) -> I32F<E2> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                if x < 0 {
                    return I32F::MIN;
                } else {
                    return I32F::MAX;
                }
            }

            if shift >= i32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i32::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u32;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        I32F { significand: x }
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_rescale<const E2: i32>(self) -> (I32F<E2>, bool) {
        let mut x = self.significand;
        let mut overflowed = false;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            overflowed |= x != 0 && shift >= x.leading_zeros() | x.leading_ones();

            if shift >= i32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i32::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u32;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        (I32F { significand: x }, overflowed)
    }

    /// Returns the nearest multiple of 2<sup>E2</sup> to `self`, rounded to the number with even least significant digits if `self` is halfway between two multiples of 2<sup>E2</sup>, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_rescale<const E2: i32>(self) -> Option<I32F<E2>> {
        let mut x = self.significand;

        if const { E > E2 } {
            let shift = const { E.wrapping_sub(E2).cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                return None;
            }

            if shift >= i32::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { E < E2 } {
            let shift = const { E2.wrapping_sub(E).cast_unsigned() };

            if shift >= i32::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift)
                };
                let round = const {
                    let shift = E2.wrapping_sub(E).cast_unsigned();

                    !(!0u32).unbounded_shl(shift.wrapping_sub(1))
                };

                let mut temp = x as u32;
                temp = (temp & mask).wrapping_add(temp >> shift & 0x1);
                temp = temp.wrapping_add(round);
                temp >>= shift;
                x >>= shift;
                x = x.wrapping_add_unsigned(temp);
            }
        }

        Some(I32F { significand: x })
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
    pub const fn add_unsigned(self, rhs: U32F<E>) -> Self {
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
    pub const fn strict_add_unsigned(self, rhs: U32F<E>) -> Self {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        if x < self.significand {
            crate::panic::add();
        }

        Self { significand: x }
    }

    /// Computes `self + rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_add_unsigned(self, rhs: U32F<E>) -> Self {
        Self {
            significand: self.significand.wrapping_add_unsigned(rhs.significand),
        }
    }

    /// Computes `self + rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_add_unsigned(self, rhs: U32F<E>) -> Self {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        if x < self.significand {
            return Self::MAX;
        }

        Self { significand: x }
    }

    /// Computes `self + rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_add_unsigned(self, rhs: U32F<E>) -> (Self, bool) {
        let x = self.significand.wrapping_add_unsigned(rhs.significand);

        (Self { significand: x }, x < self.significand)
    }

    /// Computes `self + rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_add_unsigned(self, rhs: U32F<E>) -> Option<Self> {
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
    pub const fn sub_unsigned(self, rhs: U32F<E>) -> Self {
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
    pub const fn strict_sub_unsigned(self, rhs: U32F<E>) -> Self {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        if x > self.significand {
            crate::panic::sub();
        }

        Self { significand: x }
    }

    /// Computes `self - rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_sub_unsigned(self, rhs: U32F<E>) -> Self {
        Self {
            significand: self.significand.wrapping_sub_unsigned(rhs.significand),
        }
    }

    /// Computes `self - rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_sub_unsigned(self, rhs: U32F<E>) -> Self {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        if x > self.significand {
            return Self::MIN;
        }

        Self { significand: x }
    }

    /// Computes `self - rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_sub_unsigned(self, rhs: U32F<E>) -> (Self, bool) {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        (Self { significand: x }, x > self.significand)
    }

    /// Computes `self - rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_sub_unsigned(self, rhs: U32F<E>) -> Option<Self> {
        let x = self.significand.wrapping_sub_unsigned(rhs.significand);

        if x > self.significand {
            return None;
        }

        Some(Self { significand: x })
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn mul<const R: i32>(self, rhs: I32F<R>) -> Self {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::mul();
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if cfg!(debug_assertions) && (x < i32::MIN as i64 || x > i32::MAX as i64) {
            crate::panic::mul();
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self * rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_mul<const R: i32>(self, rhs: I32F<R>) -> Self {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::mul();
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i32::MIN as i64 || x > i32::MAX as i64 {
            crate::panic::mul();
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self * rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_mul<const R: i32>(self, rhs: I32F<R>) -> Self {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self * rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_mul<const R: i32>(self, rhs: I32F<R>) -> Self {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                if x < 0 {
                    return Self::MIN;
                } else {
                    return Self::MAX;
                }
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i32::MIN as i64 {
            return Self::MIN;
        } else if x > i32::MAX as i64 {
            return Self::MAX;
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self * rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_mul<const R: i32>(self, rhs: I32F<R>) -> (Self, bool) {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);
        let mut overflowed = false;

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            overflowed |= x != 0 && shift >= x.leading_zeros() | x.leading_ones();

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        overflowed |= x < i32::MIN as i64 || x > i32::MAX as i64;

        (
            Self {
                significand: x as i32,
            },
            overflowed,
        )
    }

    /// Computes `self * rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_mul<const R: i32>(self, rhs: I32F<R>) -> Option<Self> {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                return None;
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i32::MIN as i64 || x > i32::MAX as i64 {
            return None;
        }

        Some(Self {
            significand: x as i32,
        })
    }

    /// Computes `self * rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic on overflow for debug builds, or return a wrapping result for release builds.
    #[must_use]
    #[track_caller]
    pub const fn mul_unsigned<const R: i32>(self, rhs: U32F<R>) -> Self {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::mul();
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

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

        if cfg!(debug_assertions) && (x < i32::MIN as i64 || x > i32::MAX as i64) {
            crate::panic::mul();
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self * rhs`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, even if overflow checks are disabled.
    #[must_use]
    #[track_caller]
    pub const fn strict_mul_unsigned<const R: i32>(self, rhs: U32F<R>) -> Self {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                crate::panic::mul();
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

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

        if x < i32::MIN as i64 || x > i32::MAX as i64 {
            crate::panic::mul();
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self * rhs`, wrapping around at the numeric bounds of the type.
    #[must_use]
    pub const fn wrapping_mul_unsigned<const R: i32>(self, rhs: U32F<R>) -> Self {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

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

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self * rhs`, saturating at the numeric bounds of the type instead of overflowing.
    #[must_use]
    pub const fn saturating_mul_unsigned<const R: i32>(self, rhs: U32F<R>) -> Self {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                if x < 0 {
                    return Self::MIN;
                } else {
                    return Self::MAX;
                }
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

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

        if x < i32::MIN as i64 {
            return Self::MIN;
        } else if x > i32::MAX as i64 {
            return Self::MAX;
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self * rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    #[must_use]
    pub const fn overflowing_mul_unsigned<const R: i32>(self, rhs: U32F<R>) -> (Self, bool) {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);
        let mut overflowed = false;

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            overflowed |= x != 0 && shift >= x.leading_zeros() | x.leading_ones();

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

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

        overflowed |= x < i32::MIN as i64 || x > i32::MAX as i64;

        (
            Self {
                significand: x as i32,
            },
            overflowed,
        )
    }

    /// Computes `self * rhs`, returning `None` if overflow occurred.
    #[must_use]
    pub const fn checked_mul_unsigned<const R: i32>(self, rhs: U32F<R>) -> Option<Self> {
        let mut x = (self.significand as i64).wrapping_mul(rhs.significand as i64);

        if const { R > 0 } {
            let shift = const { R.cast_unsigned() };

            if x != 0 && shift >= x.leading_zeros() | x.leading_ones() {
                return None;
            }

            if shift >= i64::BITS {
                x = 0;
            } else {
                x <<= shift;
            }
        } else if const { R < 0 } {
            let shift = const { R.wrapping_neg().cast_unsigned() };

            if shift >= i64::BITS {
                x = 0;
            } else {
                let mask = const {
                    let shift = R.wrapping_neg().cast_unsigned();

                    !(!0u64).unbounded_shl(shift)
                };
                let round = const {
                    let shift = R.wrapping_neg().cast_unsigned();

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

        if x < i32::MIN as i64 || x > i32::MAX as i64 {
            return None;
        }

        Some(Self {
            significand: x as i32,
        })
    }

    #[doc(hidden)]
    #[must_use]
    #[track_caller]
    pub const fn div<const R: i32>(self, rhs: I32F<R>) -> Self {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i64) << -OFFSET).overflowing_div(rhs.significand as i64);
        let negative = (x < 0) != overflowed;
        let mut x = x as u64;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift > x.leading_zeros() {
                crate::panic::div();
            }

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if cfg!(debug_assertions) && x > i32::MAX as u64 + negative as u64 {
            crate::panic::div();
        }

        if negative {
            x = x.wrapping_neg();
        }

        Self {
            significand: x as i32,
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
    pub const fn strict_div<const R: i32>(self, rhs: I32F<R>) -> Self {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i64) << -OFFSET).overflowing_div(rhs.significand as i64);
        let negative = (x < 0) != overflowed;
        let mut x = x as u64;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() {
                crate::panic::div();
            }

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x > (i32::MAX as u64).wrapping_add(negative as u64) {
            crate::panic::div();
        }

        if negative {
            x = x.wrapping_neg();
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self / rhs`, wrapping around at the numeric bounds of the type.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn wrapping_div<const R: i32>(self, rhs: I32F<R>) -> Self {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i64) << -OFFSET).overflowing_div(rhs.significand as i64);
        let negative = (x < 0) != overflowed;
        let mut x = x as u64;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
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
            significand: x as i32,
        }
    }

    /// Computes `self / rhs`, saturating at the numeric bounds of the type instead of overflowing.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn saturating_div<const R: i32>(self, rhs: I32F<R>) -> Self {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i64) << -OFFSET).overflowing_div(rhs.significand as i64);
        let negative = (x < 0) != overflowed;
        let mut x = x as u64;

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x > (i32::MAX as u64).wrapping_add(negative as u64) {
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
            significand: x as i32,
        }
    }

    /// Computes `self / rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn overflowing_div<const R: i32>(self, rhs: I32F<R>) -> (Self, bool) {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        let (x, overflowed) =
            ((self.significand as i64) << -OFFSET).overflowing_div(rhs.significand as i64);
        let negative = (x < 0) != overflowed;
        let mut x = x as u64;
        let mut overflowed = false;

        if negative {
            x = x.wrapping_neg();
        }

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        overflowed |= x > (i32::MAX as u64).wrapping_add(negative as u64);

        if negative {
            x = x.wrapping_neg();
        }

        (
            Self {
                significand: x as i32,
            },
            overflowed,
        )
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or overflow occurred.
    #[must_use]
    pub const fn checked_div<const R: i32>(self, rhs: I32F<R>) -> Option<Self> {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        if rhs.significand == 0 {
            return None;
        }

        let (x, overflowed) =
            ((self.significand as i64) << -OFFSET).overflowing_div(rhs.significand as i64);
        let negative = (x < 0) != overflowed;
        let mut x = x as u64;

        if negative {
            x = x.wrapping_neg();
        }

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() {
                return None;
            }

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0u64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x > (i32::MAX as u64).wrapping_add(negative as u64) {
            return None;
        }

        if negative {
            x = x.wrapping_neg();
        }

        Some(Self {
            significand: x as i32,
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
    pub const fn div_unsigned<const R: i32>(self, rhs: U32F<R>) -> Self {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        let mut x = ((self.significand as i64) << -OFFSET) / rhs.significand as i64;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if cfg!(debug_assertions) && x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                crate::panic::div();
            }

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if cfg!(debug_assertions) && (x < i32::MIN as i64 || x > i32::MAX as i64) {
            crate::panic::div();
        }

        Self {
            significand: x as i32,
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
    pub const fn strict_div_unsigned<const R: i32>(self, rhs: U32F<R>) -> Self {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        let mut x = ((self.significand as i64) << -OFFSET) / rhs.significand as i64;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                crate::panic::div();
            }

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i32::MIN as i64 || x > i32::MAX as i64 {
            crate::panic::div();
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self / rhs`, wrapping around at the numeric bounds of the type.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn wrapping_div_unsigned<const R: i32>(self, rhs: U32F<R>) -> Self {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        let mut x = ((self.significand as i64) << -OFFSET) / rhs.significand as i64;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self / rhs`, saturating at the numeric bounds of the type instead of overflowing.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn saturating_div_unsigned<const R: i32>(self, rhs: U32F<R>) -> Self {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        let mut x = ((self.significand as i64) << -OFFSET) / rhs.significand as i64;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                if x < 0 {
                    return Self::MIN;
                } else {
                    return Self::MAX;
                }
            }

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i32::MIN as i64 {
            return Self::MIN;
        } else if x > i32::MAX as i64 {
            return Self::MAX;
        }

        Self {
            significand: x as i32,
        }
    }

    /// Computes `self / rhs`. Returns a tuple of the wrapping result and a boolean indicating whether overflow occurred.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub const fn overflowing_div_unsigned<const R: i32>(self, rhs: U32F<R>) -> (Self, bool) {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        let mut x = ((self.significand as i64) << -OFFSET) / rhs.significand as i64;
        let mut overflowed = false;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            overflowed |= x != 0 && shift > x.leading_zeros() | x.leading_ones();

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        overflowed |= x < i32::MIN as i64 || x > i32::MAX as i64;

        (
            Self {
                significand: x as i32,
            },
            overflowed,
        )
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or overflow occurred.
    #[must_use]
    pub const fn checked_div_unsigned<const R: i32>(self, rhs: U32F<R>) -> Option<Self> {
        const OFFSET: i32 = i32::BITS.cast_signed() - i64::BITS.cast_signed();

        if rhs.significand == 0 {
            return None;
        }

        let mut x = ((self.significand as i64) << -OFFSET) / rhs.significand as i64;

        if const { R < OFFSET } {
            let shift = const { OFFSET.wrapping_sub(R).cast_unsigned() };

            if x != 0 && shift > x.leading_zeros() | x.leading_ones() {
                return None;
            }

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
                let round = const {
                    let shift = R.wrapping_sub(OFFSET).cast_unsigned();

                    !(!0i64).unbounded_shl(shift.wrapping_sub(1))
                };

                x = x.wrapping_add(x >> shift & 0x1);
                x = x.wrapping_add(round);
                x >>= shift;
            }
        }

        if x < i32::MIN as i64 || x > i32::MAX as i64 {
            return None;
        }

        Some(Self {
            significand: x as i32,
        })
    }
}

impl I32F<-31> {
    /// Computes `cos(π * self)` using a minimax second-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.60096 ⋅ 10<sup>-2</sup>.
    #[must_use]
    pub const fn cospi_2(self) -> I32F<-30> {
        I32F {
            significand: crate::algorithm::cospi_i32_2(self.significand),
        }
    }

    /// Computes `cos(π * self)` using a minimax fourth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.18799 ⋅ 10<sup>-4</sup>.
    #[must_use]
    pub const fn cospi_4(self) -> I32F<-30> {
        I32F {
            significand: crate::algorithm::cospi_i32_4(self.significand),
        }
    }

    /// Computes `cos(π * self)` using a minimax sixth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.20285 ⋅ 10<sup>-6</sup>.
    #[must_use]
    pub const fn cospi_6(self) -> I32F<-30> {
        I32F {
            significand: crate::algorithm::cospi_i32_6(self.significand),
        }
    }

    /// Computes `cos(π * self)` using a minimax eighth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.98045 ⋅ 10<sup>-8</sup>.
    #[must_use]
    pub const fn cospi_8(self) -> I32F<-30> {
        I32F {
            significand: crate::algorithm::cospi_i32_8(self.significand),
        }
    }

    /// Computes `cos(π * self)` using a minimax tenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 2.70068 ⋅ 10<sup>-10</sup>.
    #[must_use]
    pub const fn cospi_10(self) -> I32F<-30> {
        I32F {
            significand: crate::algorithm::cospi_i32_10(self.significand),
        }
    }

    /// Computes `sin(π * self)` using a minimax second-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.60096 ⋅ 10<sup>-2</sup>.
    #[must_use]
    pub const fn sinpi_2(self) -> I32F<-30> {
        I32F {
            significand: crate::algorithm::cospi_i32_2(
                self.significand.wrapping_add_unsigned(0xC0000000),
            ),
        }
    }

    /// Computes `sin(π * self)` using a minimax fourth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.18799 ⋅ 10<sup>-4</sup>.
    #[must_use]
    pub const fn sinpi_4(self) -> I32F<-30> {
        I32F {
            significand: crate::algorithm::cospi_i32_4(
                self.significand.wrapping_add_unsigned(0xC0000000),
            ),
        }
    }

    /// Computes `sin(π * self)` using a minimax sixth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 9.20285 ⋅ 10<sup>-6</sup>.
    #[must_use]
    pub const fn sinpi_6(self) -> I32F<-30> {
        I32F {
            significand: crate::algorithm::cospi_i32_6(
                self.significand.wrapping_add_unsigned(0xC0000000),
            ),
        }
    }

    /// Computes `sin(π * self)` using a minimax eighth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 5.98045 ⋅ 10<sup>-8</sup>.
    #[must_use]
    pub const fn sinpi_8(self) -> I32F<-30> {
        I32F {
            significand: crate::algorithm::cospi_i32_8(
                self.significand.wrapping_add_unsigned(0xC0000000),
            ),
        }
    }

    /// Computes `sin(π * self)` using a minimax tenth-order Taylor series approximation, where `self` is in half-turns. The error is bounded by 2.70068 ⋅ 10<sup>-10</sup>.
    #[must_use]
    pub const fn sinpi_10(self) -> I32F<-30> {
        I32F {
            significand: crate::algorithm::cospi_i32_10(
                self.significand.wrapping_add_unsigned(0xC0000000),
            ),
        }
    }
}

impl From<I32F<0>> for i32 {
    fn from(value: I32F<0>) -> Self {
        value.significand
    }
}

impl From<i32> for I32F<0> {
    fn from(value: i32) -> Self {
        Self { significand: value }
    }
}

impl<const E: i32> From<I8F<E>> for I32F<E> {
    fn from(value: I8F<E>) -> Self {
        Self::from_i8f(value)
    }
}

impl<const E: i32> From<I16F<E>> for I32F<E> {
    fn from(value: I16F<E>) -> Self {
        Self::from_i16f(value)
    }
}

impl<const E: i32> From<U8F<E>> for I32F<E> {
    fn from(value: U8F<E>) -> Self {
        Self::from_u8f(value)
    }
}

impl<const E: i32> From<U16F<E>> for I32F<E> {
    fn from(value: U16F<E>) -> Self {
        Self::from_u16f(value)
    }
}

impl<const E1: i32, const E2: i32> PartialEq<I32F<E2>> for I32F<E1> {
    fn eq(&self, other: &I32F<E2>) -> bool {
        let mut lhs = self.significand;
        let mut rhs = other.significand;

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
        let mut lhs = self.significand;
        let mut rhs = other.significand;

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
        f.debug_tuple(">").field(&self.significand).finish()
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
        Self::neg(self)
    }
}

impl<const E: i32> ops::Add for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self::add(self, rhs)
    }
}

impl<const E: i32> ops::Add<U32F<E>> for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: U32F<E>) -> Self::Output {
        Self::add_unsigned(self, rhs)
    }
}

impl<const E: i32> ops::Sub for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::sub(self, rhs)
    }
}

impl<const E: i32> ops::Sub<U32F<E>> for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: U32F<E>) -> Self::Output {
        Self::sub_unsigned(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Mul<I32F<R>> for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn mul(self, rhs: I32F<R>) -> Self::Output {
        Self::mul(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Mul<U32F<R>> for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn mul(self, rhs: U32F<R>) -> Self::Output {
        Self::mul_unsigned(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Div<I32F<R>> for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn div(self, rhs: I32F<R>) -> Self::Output {
        Self::div(self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::Div<U32F<R>> for I32F<E> {
    type Output = Self;

    #[track_caller]
    fn div(self, rhs: U32F<R>) -> Self::Output {
        Self::div_unsigned(self, rhs)
    }
}

impl<const E: i32> ops::AddAssign for I32F<E> {
    #[track_caller]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(*self, rhs)
    }
}

impl<const E: i32> ops::AddAssign<U32F<E>> for I32F<E> {
    #[track_caller]
    fn add_assign(&mut self, rhs: U32F<E>) {
        *self = Self::add_unsigned(*self, rhs)
    }
}

impl<const E: i32> ops::SubAssign for I32F<E> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::sub(*self, rhs)
    }
}

impl<const E: i32> ops::SubAssign<U32F<E>> for I32F<E> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: U32F<E>) {
        *self = Self::sub_unsigned(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::MulAssign<I32F<R>> for I32F<E> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: I32F<R>) {
        *self = Self::mul(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::MulAssign<U32F<R>> for I32F<E> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: U32F<R>) {
        *self = Self::mul_unsigned(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::DivAssign<I32F<R>> for I32F<E> {
    #[track_caller]
    fn div_assign(&mut self, rhs: I32F<R>) {
        *self = Self::div(*self, rhs)
    }
}

impl<const E: i32, const R: i32> ops::DivAssign<U32F<R>> for I32F<E> {
    #[track_caller]
    fn div_assign(&mut self, rhs: U32F<R>) {
        *self = Self::div_unsigned(*self, rhs)
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
    fn cospi_2_exact_right_angles() {
        assert_eq!(I32F::<-31>::from_bits(0x0).cospi_2(), I32F::<0>::new(1));
        assert_eq!(
            I32F::<-31>::from_bits(0x40000000).cospi_2(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0x80000000).cospi_2(),
            I32F::<0>::new(-1)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0xC0000000).cospi_2(),
            I32F::<0>::new(0)
        );
    }

    #[test]
    fn cospi_4_exact_right_angles() {
        assert_eq!(I32F::<-31>::from_bits(0x0).cospi_4(), I32F::<0>::new(1));
        assert_eq!(
            I32F::<-31>::from_bits(0x40000000).cospi_4(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0x80000000).cospi_4(),
            I32F::<0>::new(-1)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0xC0000000).cospi_4(),
            I32F::<0>::new(0)
        );
    }

    #[test]
    fn cospi_6_exact_right_angles() {
        assert_eq!(I32F::<-31>::from_bits(0x0).cospi_6(), I32F::<0>::new(1));
        assert_eq!(
            I32F::<-31>::from_bits(0x40000000).cospi_6(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0x80000000).cospi_6(),
            I32F::<0>::new(-1)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0xC0000000).cospi_6(),
            I32F::<0>::new(0)
        );
    }

    #[test]
    fn cospi_8_exact_right_angles() {
        assert_eq!(I32F::<-31>::from_bits(0x0).cospi_8(), I32F::<0>::new(1));
        assert_eq!(
            I32F::<-31>::from_bits(0x40000000).cospi_8(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0x80000000).cospi_8(),
            I32F::<0>::new(-1)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0xC0000000).cospi_8(),
            I32F::<0>::new(0)
        );
    }

    #[test]
    fn cospi_10_exact_right_angles() {
        assert_eq!(I32F::<-31>::from_bits(0x0).cospi_10(), I32F::<0>::new(1));
        assert_eq!(
            I32F::<-31>::from_bits(0x40000000).cospi_10(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0x80000000).cospi_10(),
            I32F::<0>::new(-1)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0xC0000000).cospi_10(),
            I32F::<0>::new(0)
        );
    }

    #[test]
    fn div_one_divide_by_eighth() {
        assert_eq!(
            I32F::<-16>::from_bits(0x10000) / I32F::<-33>::from_bits(0x40000000),
            I32F::<-16>::from_bits(0x80000)
        );
        assert_eq!(
            I32F::<-16>::from_bits(0x10000) / I32F::<-32>::from_bits(0x20000000),
            I32F::<-16>::from_bits(0x80000)
        );
        assert_eq!(
            I32F::<-16>::from_bits(0x10000) / I32F::<-3>::new(1),
            I32F::<-16>::from_bits(0x80000)
        );
    }

    #[test]
    fn div_one_divide_by_three() {
        assert_eq!(
            I32F::<-30>::from_bits(0x40000000) / I32F::<0>::new(3),
            I32F::<-30>::from_bits(0x15555555)
        );
    }

    #[test]
    fn div_one_divide_by_two() {
        assert_eq!(
            I32F::<-16>::from_bits(0x10000) / I32F::<1>::new(1),
            I32F::<-16>::from_bits(0x8000)
        );
    }

    #[test]
    fn div_min_divide_negative_two_could_overflow() {
        assert_eq!(
            I32F::<0>::MIN / I32F::<1>::new(-1),
            I32F::<0>::from_bits(0x40000000)
        );
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
    fn scale_round_ties_even() {
        assert_eq!(
            I32F::<-16>::from_bits(0x8000).rescale::<0>(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-16>::from_bits(0x8001).rescale::<0>(),
            I32F::<0>::new(1)
        );
        assert_eq!(
            I32F::<-16>::from_bits(0x17FFF).rescale::<0>(),
            I32F::<0>::new(1)
        );
        assert_eq!(
            I32F::<-16>::from_bits(0x18000).rescale::<0>(),
            I32F::<0>::new(2)
        );
    }

    #[test]
    fn sinpi_2_exact_right_angles() {
        assert_eq!(I32F::<-31>::from_bits(0x0).sinpi_2(), I32F::<0>::new(0));
        assert_eq!(
            I32F::<-31>::from_bits(0x40000000).sinpi_2(),
            I32F::<0>::new(1)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0x80000000).sinpi_2(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0xC0000000).sinpi_2(),
            I32F::<0>::new(-1)
        );
    }

    #[test]
    fn sinpi_4_exact_right_angles() {
        assert_eq!(I32F::<-31>::from_bits(0x0).sinpi_4(), I32F::<0>::new(0));
        assert_eq!(
            I32F::<-31>::from_bits(0x40000000).sinpi_4(),
            I32F::<0>::new(1)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0x80000000).sinpi_4(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0xC0000000).sinpi_4(),
            I32F::<0>::new(-1)
        );
    }

    #[test]
    fn sinpi_6_exact_right_angles() {
        assert_eq!(I32F::<-31>::from_bits(0x0).sinpi_6(), I32F::<0>::new(0));
        assert_eq!(
            I32F::<-31>::from_bits(0x40000000).sinpi_6(),
            I32F::<0>::new(1)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0x80000000).sinpi_6(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0xC0000000).sinpi_6(),
            I32F::<0>::new(-1)
        );
    }

    #[test]
    fn sinpi_8_exact_right_angles() {
        assert_eq!(I32F::<-31>::from_bits(0x0).sinpi_8(), I32F::<0>::new(0));
        assert_eq!(
            I32F::<-31>::from_bits(0x40000000).sinpi_8(),
            I32F::<0>::new(1)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0x80000000).sinpi_8(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0xC0000000).sinpi_8(),
            I32F::<0>::new(-1)
        );
    }

    #[test]
    fn sinpi_10_exact_right_angles() {
        assert_eq!(I32F::<-31>::from_bits(0x0).sinpi_10(), I32F::<0>::new(0));
        assert_eq!(
            I32F::<-31>::from_bits(0x40000000).sinpi_10(),
            I32F::<0>::new(1)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0x80000000).sinpi_10(),
            I32F::<0>::new(0)
        );
        assert_eq!(
            I32F::<-31>::from_bits(0xC0000000).sinpi_10(),
            I32F::<0>::new(-1)
        );
    }

    #[test]
    fn to_f32_half() {
        assert_eq!(I32F::<-1>::new(1).to_f32(), 0.5);
    }

    #[test]
    fn to_f32_max() {
        assert!(I32F::<{ f32::MAX_EXP }>::new(1).to_f32() > f32::MAX);
        assert_eq!(
            I32F::<{ f32::MAX_EXP - 24 }>::from_bits(0xFFFFFF).to_f32(),
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
        let x = 0x1000005;
        assert_eq!(I32F::<-152>::from_bits(x).to_f32().to_bits(), 0x200001);
        assert_eq!(
            (x as f32 * 2.0f32.powi(-76) * 2.0f32.powi(-76)).to_bits(),
            0x200000
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

    #[test]
    fn try_new_from_f32_negative_one() {
        assert_eq!(I32F::<0>::try_new_from_f32(-1.0), Ok(I32F::new(-1)));
    }

    #[test]
    fn try_new_from_f32_one() {
        assert_eq!(I32F::<0>::try_new_from_f32(1.0), Ok(I32F::new(1)));
    }

    #[test]
    fn try_new_from_f32_zero() {
        assert_eq!(
            I32F::<{ i32::MIN }>::try_new_from_f32(0.0),
            Ok(I32F::new(0))
        );
        assert_eq!(I32F::<0>::try_new_from_f32(0.0), Ok(I32F::new(0)));
        assert_eq!(
            I32F::<{ i32::MAX }>::try_new_from_f32(0.0),
            Ok(I32F::new(0))
        );
    }

    #[test]
    fn mul_three_times_half() {
        assert_eq!(
            I32F::<-16>::from_bits(0x30000) * I32F::<-1>::new(1),
            I32F::<-16>::from_bits(0x18000)
        );
    }

    #[test]
    fn mul_three_times_three_sixteenths() {
        assert_eq!(I32F::<0>::new(3) * I32F::<-4>::new(3), I32F::<0>::new(1));
    }

    #[test]
    fn mul_three_times_two() {
        assert_eq!(
            I32F::<-16>::from_bits(0x30000) * I32F::<1>::new(1),
            I32F::<-16>::from_bits(0x60000)
        );
    }
}
