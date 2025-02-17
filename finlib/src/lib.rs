//! # Quant finance functionality for Rust with FFIs to C/C++, C#, Python and WASM

pub mod interest;
pub mod stats;
pub mod util;
pub mod risk;
pub mod ffi;
pub mod options;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[macro_export]
macro_rules! gated_iter {
    ($x:expr) => {
        {
            $x.iter()
        }
    };
}

#[macro_export]
macro_rules! gated_iter_mut {
    ($x:expr) => {
        {
            if cfg!(feature = "parallel") {
                $x.par_iter_mut()
            }
            else {
                $x.iter_mut()
            }
        }
    };
}