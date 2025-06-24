//! # Quant finance functionality for Rust with FFIs to C/C++, C#, Python and WASM

#![cfg_attr(feature = "btree_cursors", feature(btree_cursors))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod curve;
pub mod derivatives;
pub mod errors;
pub mod ffi;
pub mod fixed_income;
pub mod indicators;
pub mod interest;
pub mod portfolio;
pub mod price;
pub mod risk;
pub mod stats;
pub mod util;
