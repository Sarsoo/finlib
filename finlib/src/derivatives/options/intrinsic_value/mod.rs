use crate::derivatives::options::Moneyness;
use crate::derivatives::options::Moneyness::{AtTheMoney, InTheMoney, OutOfTheMoney};
use crate::derivatives::TradeSide;
use crate::price::enums::Side;
use crate::price::payoff::{Payoff, Premium, Profit};
use crate::{impl_premium, impl_premium_profit, impl_side};
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
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct CallOption {
    pub strike: f64,
    pub premium: f64,
    pub side: Side,
}

impl_side!(CallOption);
impl_premium!(CallOption);
impl_premium_profit!(f64, CallOption);

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

impl Payoff<f64> for CallOption {
    fn payoff(&self, underlying: f64) -> f64 {
        self.value(underlying).max(0.)
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PutOption {
    pub strike: f64,
    pub premium: f64,
    pub side: Side,
}

impl_side!(PutOption);
impl_premium!(PutOption);
impl_premium_profit!(f64, PutOption);

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

impl Payoff<f64> for PutOption {
    fn payoff(&self, underlying: f64) -> f64 {
        self.value(underlying).max(0.)
    }
}
