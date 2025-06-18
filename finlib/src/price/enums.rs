#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Side {
    Buy, Sell
}