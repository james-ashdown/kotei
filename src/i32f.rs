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
