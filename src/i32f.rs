use ::core::fmt;

#[derive(Clone, Copy, Hash)]
pub struct I32F<const E: i32>(pub(crate) i32);

impl<const E: i32> I32F<E> {
    pub const fn new(significand: i32) -> Self {
        Self(significand)
    }

    pub const fn significand(self) -> i32 {
        self.0
    }

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
