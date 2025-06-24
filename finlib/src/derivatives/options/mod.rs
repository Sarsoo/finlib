#[cfg(feature = "std")]
pub mod blackscholes;
pub mod greeks;
pub mod intrinsic_value;
pub mod option_contract;
pub mod templates;

pub use greeks::*;
pub use option_contract::*;

use crate::derivatives::TradeSide;
use crate::price::payoff::Profit;
use crate::price::payoff::{Payoff, Premium};
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

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum OptionStyle {
    European,
    American,
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

pub trait IOption: Send + TradeSide + Payoff<f64> + Profit<f64> + Premium {
    fn option_type(&self) -> OptionType;
    fn option_style(&self) -> OptionStyle;
    fn price(&self) -> f64;
    fn strike(&self) -> f64;
}
