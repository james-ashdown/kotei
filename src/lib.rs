//! `kotei` is a human-readable, no-dependencies `no_std` fixed-point arithmetic library.

#![warn(missing_docs)]
#![no_std]

mod algorithm;
pub mod error;
mod panic;

mod i8f;
pub use i8f::*;

mod i16f;
pub use i16f::*;

mod i32f;
pub use i32f::*;

mod i64f;
pub use i64f::*;

mod i128f;
pub use i128f::*;

mod u8f;
pub use u8f::*;

mod u16f;
pub use u16f::*;

mod u32f;
pub use u32f::*;

mod u64f;
pub use u64f::*;

mod u128f;
pub use u128f::*;
