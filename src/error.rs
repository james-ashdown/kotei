//! Error-handling API.

use ::core::error;
use ::core::fmt;

/// The error type returned when a checked floating-point number type conversion fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TryFromFloatError {
    /// Value was not a number.
    Nan,
    /// Value exceeded the maximum or minimum value for the target fixed-point type.
    Overflow,
}

impl error::Error for TryFromFloatError {}

impl fmt::Display for TryFromFloatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            TryFromFloatError::Overflow => "value exceeded target boundary",
            TryFromFloatError::Nan => "value is not a number",
        };

        f.write_str(message)
    }
}
