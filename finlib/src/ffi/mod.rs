//! FFI specific functionality to define the struct function interfaces in Python and WASM

#[cfg(feature = "py")]
pub mod py;
#[cfg(feature = "wasm")]
pub mod wasm;