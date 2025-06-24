//! FFI specific functionality to define the struct function interfaces in Python and WASM

#[cfg(feature = "py")]
pub mod py;
// #[cfg(feature = "wasm")]
#[cfg(all(
    target_arch = "wasm32",
    feature = "wasm",
    not(any(target_os = "emscripten", target_os = "wasi", target_os = "linux"))
))]
pub mod wasm;
