#![cfg_attr(feature = "btree_cursors", feature(btree_cursors))]
//! # Quant finance functionality for Rust with FFIs to C/C++, C#, Python and WASM

pub mod curve;
pub mod derivatives;
pub mod ffi;
pub mod indicators;
pub mod interest;
pub mod portfolio;
pub mod price;
pub mod risk;
pub mod stats;
pub mod util;
