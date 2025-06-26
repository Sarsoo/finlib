use crate::price::enums::Side;
use chrono::{DateTime, Utc};
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub trait IPrice {
    fn value(&self) -> f64;
    fn side_value(&self, side: Side) -> Result<f64, ()>;
    fn time(&self) -> Result<DateTime<Utc>, ()>;
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub struct Price {
    pub value: f64,
    pub side: Side,
}

impl IPrice for Price {
    fn value(&self) -> f64 {
        self.value
    }

    fn side_value(&self, side: Side) -> Result<f64, ()> {
        if side == self.side {
            return Ok(self.value());
        }
        Err(())
    }

    fn time(&self) -> Result<DateTime<Utc>, ()> {
        Err(())
    }
}
