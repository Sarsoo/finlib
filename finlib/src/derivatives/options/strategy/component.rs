use crate::derivatives::options::strategy::IOptionStrategyComponent;
use crate::derivatives::options::OptionType;
use crate::derivatives::TradeSide;
use crate::price::enums::Side;
use crate::price::payoff::{Payoff, Premium, Profit};
use crate::{impl_premium, impl_premium_profit, impl_side};
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OptionStrategyComponent {
    pub option_type: OptionType,
    pub side: Side,
    pub strike: f64,
    pub premium: f64,
}

impl OptionStrategyComponent {
    pub fn from(option_type: OptionType, side: Side, strike: f64, premium: f64) -> Self {
        Self {
            option_type,
            side,
            strike,
            premium,
        }
    }
}

impl_side!(OptionStrategyComponent);
impl_premium!(OptionStrategyComponent);
impl_premium_profit!(f64, OptionStrategyComponent);

impl Payoff<f64> for OptionStrategyComponent {
    fn payoff(&self, underlying: f64) -> f64 {
        match (self.option_type, self.side) {
            (OptionType::Call, Side::Buy) => (underlying - self.strike).max(0.0),
            (OptionType::Call, Side::Sell) => -(underlying - self.strike).max(0.0),
            (OptionType::Put, Side::Buy) => (self.strike - underlying).max(0.0),
            (OptionType::Put, Side::Sell) => -(self.strike - underlying).max(0.0),
        }
    }
}

impl IOptionStrategyComponent for OptionStrategyComponent {
    fn option_type(&self) -> OptionType {
        self.option_type
    }

    fn strike(&self) -> f64 {
        self.strike
    }

    fn will_be_exercised(&self, underlying: f64) -> bool {
        match self.option_type {
            OptionType::Call => self.strike < underlying,
            OptionType::Put => self.strike > underlying,
        }
    }
}
