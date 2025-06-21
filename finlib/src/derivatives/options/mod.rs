pub mod blackscholes;
pub mod intrinsic_value;
pub mod strategy;

#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum OptionType {
    Call,
    Put,
}

pub trait Option {
    fn price(&self) -> f64;
    fn strike(&self) -> f64;
}

pub trait Greeks: Option {
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct OptionGreeks {
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
    pub rho: f64,
}

impl OptionGreeks {
    pub fn from(option: &impl Greeks) -> Self {
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Moneyness {
    InTheMoney,
    AtTheMoney,
    OutOfTheMoney,
}
