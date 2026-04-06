//! `kotei` is a human-readable, no-dependencies `no_std` fixed-point arithmetic library.

#![warn(missing_docs)]
#![no_std]

mod algorithm;
pub mod error;
mod panic;

mod i32f;
pub use i32f::*;

mod i64f;
pub use i64f::*;

mod i128f;
pub use i128f::*;

mod u32f;
pub use u32f::*;

mod u64f;
pub use u64f::*;

mod u128f;
pub use u128f::*;
