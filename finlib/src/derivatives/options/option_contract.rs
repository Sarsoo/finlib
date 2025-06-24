#[cfg(feature = "std")]
use crate::derivatives::options::blackscholes::OptionVariables;
use crate::derivatives::options::{IOption, OptionGreeks, OptionStyle, OptionType};
use crate::derivatives::TradeSide;
use crate::price::enums::Side;
use crate::price::payoff::Payoff;
use crate::price::payoff::Premium;
use crate::price::payoff::Profit;
use crate::{impl_premium, impl_premium_profit, impl_side};
use bon::Builder;
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
#[derive(Builder, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct OptionContract {
    pub option_type: OptionType,
    pub option_style: OptionStyle,
    pub strike: f64,
    pub premium: f64,
    pub side: Side,
    pub greeks: Option<OptionGreeks>,
}

impl_side!(OptionContract);
impl_premium!(OptionContract);
impl_premium_profit!(f64, OptionContract);

impl Payoff<f64> for OptionContract {
    fn payoff(&self, underlying: f64) -> f64 {
        match (self.option_type, self.side) {
            (OptionType::Call, Side::Buy) => (underlying - self.strike).max(0.0),
            (OptionType::Call, Side::Sell) => -(underlying - self.strike).max(0.0),
            (OptionType::Put, Side::Buy) => (self.strike - underlying).max(0.0),
            (OptionType::Put, Side::Sell) => -(self.strike - underlying).max(0.0),
        }
    }
}

impl IOption for OptionContract {
    fn option_type(&self) -> OptionType {
        self.option_type
    }

    fn option_style(&self) -> OptionStyle {
        self.option_style
    }

    fn price(&self) -> f64 {
        self.premium
    }

    fn strike(&self) -> f64 {
        self.strike
    }
}

impl OptionContract {
    pub fn from(
        option_type: OptionType,
        option_style: OptionStyle,
        side: Side,
        strike: f64,
        premium: f64,
    ) -> Self {
        Self {
            option_type,
            option_style,
            side,
            strike,
            premium,
            greeks: None,
        }
    }

    #[cfg(feature = "std")]
    pub fn from_vars(
        vars: &OptionVariables,
        option_type: OptionType,
        option_style: OptionStyle,
        side: Side,
    ) -> Self {
        Self::builder()
            .option_type(option_type)
            .option_style(option_style)
            .side(side)
            .premium(f64::NAN)
            .strike(vars.strike_price)
            .build()
    }

    pub fn will_be_exercised(&self, underlying: f64) -> bool {
        match self.option_type {
            OptionType::Call => self.strike < underlying,
            OptionType::Put => self.strike > underlying,
        }
    }
}
