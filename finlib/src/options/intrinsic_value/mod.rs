use crate::options::Moneyness;
use crate::options::Moneyness::{AtTheMoney, InTheMoney, OutOfTheMoney};
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub trait IntrinsicValue {
    fn value(&self, underlying_value: f64) -> f64;
    fn moneyness(&self, underlying_value: f64) -> Moneyness;
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct CallOption {
    pub strike: f64,
}

impl IntrinsicValue for CallOption {
    fn value(&self, underlying_value: f64) -> f64 {
        underlying_value - self.strike
    }

    fn moneyness(&self, underlying_value: f64) -> Moneyness {
        let val = self.value(underlying_value);
        if val < 0. {
            OutOfTheMoney
        } else if val == 0. {
            AtTheMoney
        } else {
            InTheMoney
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct PutOption {
    pub strike: f64,
}

impl IntrinsicValue for PutOption {
    fn value(&self, underlying_value: f64) -> f64 {
        self.strike - underlying_value
    }

    fn moneyness(&self, underlying_value: f64) -> Moneyness {
        let val = self.value(underlying_value);
        if val < 0. {
            OutOfTheMoney
        } else if val == 0. {
            AtTheMoney
        } else {
            InTheMoney
        }
    }
}
