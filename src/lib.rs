//! `kotei` is a human-readable, no-dependencies `no_std` fixed-point arithmetic library.

#![warn(missing_docs)]
#![no_std]

mod panic;

mod i32f;
pub use i32f::*;
