//! Implements arbitrary-precision arithmetic (big numbers).
//! The following numeric types are supported:
//!
//! ```ignore
//! Int    signed integers
//! ```
//!
//mod arith;
mod int;
//mod nat;

/// The largest number base accepted for string conversions.
pub const MAX_BASE: u8 = 10 + (b'z' - b'a' + 1) + (b'Z' - b'A' + 1);

pub use int::*;

//pub type Word = usize;
