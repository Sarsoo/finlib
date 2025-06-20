pub mod blackscholes;
pub mod intrinsic_value;
pub mod strategy;

#[cfg(feature = "py")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum OptionType {
    Call,
    Put,
}

pub trait Option {
    fn price(&self) -> f64;
    fn strike(&self) -> f64;

    fn delta(&self) -> f64;
    fn gamma(&self) -> f64;
    fn vega(&self) -> f64;
    fn theta(&self) -> f64;
    fn rho(&self) -> f64;

    fn calc_greeks(&mut self);
    fn has_greeks(&self) -> bool;
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct OptionGreeks {
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
    pub rho: f64,
}

impl OptionGreeks {
    pub fn from(option: &impl Option) -> Self {
        Self {
            delta: option.delta(),
            gamma: option.gamma(),
            vega: option.vega(),
            theta: option.theta(),
            rho: option.rho(),
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Moneyness {
    InTheMoney,
    AtTheMoney,
    OutOfTheMoney,
}
