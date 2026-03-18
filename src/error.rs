//! Error-handling API.

use ::core::error;
use ::core::fmt;

/// The error type returned when a checked floating-point number type conversion fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TryFromFloatError {
    /// Value was not a number.
    Nan,
    /// Value was less than the minimum value for the target fixed-point type.
    Underflow,
    /// Value was greater than the maximum value for the target fixed-point type.
    Overflow,
}

impl error::Error for TryFromFloatError {}

impl fmt::Display for TryFromFloatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            TryFromFloatError::Underflow => "value is less than target MIN",
            TryFromFloatError::Overflow => "value is greater than target MAX",
            TryFromFloatError::Nan => "value is not a number",
        };

        f.write_str(message)
    }
}
