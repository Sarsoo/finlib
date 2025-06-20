use crate::derivatives::options::strategy::IOptionStrategyComponent;
use crate::derivatives::options::OptionType;
use crate::impl_premium_profit;
use crate::price::enums::Side;
use crate::price::payoff::{Payoff, Premium, Profit};
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
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

impl_premium_profit!(f64, OptionStrategyComponent);

impl Premium for OptionStrategyComponent {
    fn premium(&self) -> f64 {
        self.premium
    }

    fn side(&self) -> Side {
        self.side
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
