use ::core::fmt;

/// The 32-bit signed fixed-point type.
#[derive(Clone, Copy, Hash)]
pub struct I32F<const E: i32>(pub(crate) i32);

impl<const E: i32> I32F<E> {
    /// Creates a new fixed-point number from an integer significand, equal to `significand` ⋅ 2<sup>E</sup>.
    pub const fn new(significand: i32) -> Self {
        Self(significand)
    }

    /// Returns the fixed-point significand, equal to `self` ⋅ 2<sup>-E</sup>.
    pub const fn significand(self) -> i32 {
        self.0
    }

    /// Returns the fixed-point exponent.
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
