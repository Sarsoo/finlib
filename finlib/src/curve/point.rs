use chrono::NaiveDate;
// #[cfg(feature = "wasm")]
// use wasm_bindgen::prelude::*;
#[cfg(feature = "py")]
use pyo3::prelude::*;

// #[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass)]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CurvePoint {
    pub bid_rate: f64,
    pub offer_rate: f64,
    pub date: NaiveDate,
}
