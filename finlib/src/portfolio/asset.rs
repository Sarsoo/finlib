use crate::price::payoff::{Payoff, Profit};
use crate::risk::var::varcovar::value_at_risk_percent;
use crate::risk::var::ValueAtRisk;
use crate::stats;
use crate::stats::{MuSigma, PopulationStats};
use crate::util::roc::rates_of_change;
use log::info;
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
pub enum ValueType {
    Absolute,
    RateOfChange,
}

/// Describes a single instrument as a list of previous values with an associated portfolio proportion
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PortfolioAsset {
    // pub portfolio_weight: f64,
    name: String,
    pub quantity: f64,
    pub(super) market_values: Vec<f64>,
    pub value_at_position_open: Option<f64>,
    pub value_type: ValueType,
}

impl PortfolioAsset {
    pub fn new(
        // portfolio_weight: f64,
        name: String,
        quantity: f64,
        market_values: Vec<f64>,
    ) -> PortfolioAsset {
        PortfolioAsset {
            // portfolio_weight,
            name,
            quantity,
            value_at_position_open: None,
            market_values,
            value_type: ValueType::Absolute,
        }
    }

    pub fn current_value(&self) -> f64 {
        *self.market_values.last().unwrap()
    }

    pub fn current_total_value(&self) -> f64 {
        self.quantity * self.current_value()
    }

    pub fn profit_loss(&self) -> Option<f64> {
        match self.value_at_position_open {
            None => None,
            Some(vo) => Some((self.current_value() - vo) * self.quantity),
        }
    }

    /// If the asset's values have been given as absolute values, convert those to a percentage change between each
    pub fn apply_rates_of_change(&mut self) {
        match self.value_type {
            ValueType::Absolute => {
                self.market_values = rates_of_change(&self.market_values).collect();
                self.value_type = ValueType::RateOfChange;
            }
            _ => {}
        }
    }
}

impl PopulationStats for PortfolioAsset {
    /// Get the mean and standard deviation of the rates of change of an asset
    ///
    /// returns (mean, std_dev)
    fn mean_and_std_dev(&self) -> Result<MuSigma, ()> {
        match self.value_type {
            ValueType::Absolute => {
                info!(
                    "[{}] Asset's values are currently absolute, calculating rates of change first",
                    self.name
                );
                let roc = rates_of_change(&self.market_values).collect::<Vec<f64>>();
                Ok(MuSigma {
                    mean: stats::mean(&roc),
                    std_dev: stats::sample_std_dev(&roc),
                })
            }
            ValueType::RateOfChange => Ok(MuSigma {
                mean: stats::mean(&self.market_values),
                std_dev: stats::sample_std_dev(&self.market_values),
            }),
        }
    }
}

impl Payoff<Option<f64>> for PortfolioAsset {
    fn payoff(&self, underlying: Option<f64>) -> f64 {
        match (underlying, self.value_at_position_open) {
            (None, _) => self.profit_loss().unwrap_or(0.0),
            (Some(u), Some(i)) => (u - i) * self.quantity,
            (_, None) => 0.0,
        }
    }
}

impl Profit<Option<f64>> for PortfolioAsset {
    fn profit(&self, underlying: Option<f64>) -> f64 {
        self.payoff(underlying)
    }
}

impl ValueAtRisk for PortfolioAsset {
    fn value_at_risk_pct(&self, confidence: f64) -> Result<f64, ()> {
        Ok(value_at_risk_percent(&self.market_values, confidence))
    }
}
