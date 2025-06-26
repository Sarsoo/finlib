use crate::price::payoff::{Payoff, Profit};
#[cfg(feature = "std")]
use crate::risk::var::varcovar::value_at_risk_from_initial_investment;
use crate::risk::var::ValueAtRisk;
use crate::stats;
use crate::stats::{MuSigma, PopulationStats};
use crate::util::roc::rates_of_change;
use bon::Builder;
use log::info;
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::market_data::price_range::{PriceRangePair, PriceTimestamp};
use crate::market_data::price_timeline::static_timeline::StaticPriceTimeline;
use crate::market_data::price_timeline::PriceTimeline;
use crate::market_data::TimeSpan;
use crate::price::{IPrice, PricePair, Side};
use crate::risk::volatility::Volatility;
use alloc::string::String;
use alloc::vec::Vec;
use chrono::{DateTime, Utc};

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
#[derive(Builder, Clone, Debug, PartialEq, PartialOrd)]
pub struct PortfolioAsset {
    pub(super) name: String,
    pub quantity: f64,
    pub(super) market_values: StaticPriceTimeline,
    pub value_at_position_open: Option<f64>,
    pub value_type: ValueType,
}

impl PortfolioAsset {
    pub fn new(name: String, quantity: f64, market_data_scale: TimeSpan) -> PortfolioAsset {
        PortfolioAsset {
            name,
            quantity,
            value_at_position_open: None,
            market_values: StaticPriceTimeline::new(market_data_scale),
            value_type: ValueType::Absolute,
        }
    }

    pub fn current_value(&self) -> Result<PriceRangePair, ()> {
        self.market_values.last()
    }

    pub fn current_total_value(&self) -> Result<f64, ()> {
        match (self.current_value(), self.quantity > 0.) {
            (Ok(v), true) => match v.side_value(Side::Sell) {
                Ok(v) => Ok(v * self.quantity),
                Err(_) => Err(()),
            },
            (Ok(v), false) => match v.side_value(Side::Sell) {
                Ok(v) => Ok(v * self.quantity),
                Err(_) => Err(()),
            },
            (Err(_), _) => Err(()),
        }
    }

    pub fn profit_loss(&self) -> Result<f64, ()> {
        match self.value_at_position_open {
            None => Err(()),
            Some(vo) => match self.current_total_value() {
                Ok(v) => Ok(v - (vo * self.quantity)),
                Err(_) => Err(()),
            },
        }
    }

    pub fn get_rates_of_change(&self) -> Result<Vec<f64>, ()> {
        match (self.value_type, self.quantity > 0.) {
            (ValueType::Absolute, true) => match &self.market_values.closing_prices(Side::Sell) {
                Ok(vals) => Ok(rates_of_change(vals).collect()),
                Err(_) => Err(()),
            },
            (ValueType::Absolute, false) => match &self.market_values.closing_prices(Side::Buy) {
                Ok(vals) => Ok(rates_of_change(vals).collect()),
                Err(_) => Err(()),
            },
            (ValueType::RateOfChange, true) => match &self.market_values.closing_prices(Side::Sell)
            {
                Ok(vals) => Ok(vals.clone()),
                Err(_) => Err(()),
            },
            (ValueType::RateOfChange, false) => match &self.market_values.closing_prices(Side::Buy)
            {
                Ok(vals) => Ok(vals.clone()),
                Err(_) => Err(()),
            },
        }
    }

    pub fn add_price(&mut self, price: PriceTimestamp) -> Result<(), ()> {
        self.market_values.add_price(price)
    }
    pub fn add_price_pair(&mut self, price: PricePair, time: DateTime<Utc>) -> Result<(), ()> {
        self.market_values.add_price_pair(price, time)
    }
}

impl PopulationStats for PortfolioAsset {
    /// Get the mean and standard deviation of the rates of change of an asset
    ///
    /// returns (mean, std_dev)
    fn mean_and_std_dev(&self) -> Result<MuSigma, ()> {
        match (self.value_type, self.quantity > 0.) {
            (ValueType::Absolute, true) => {
                info!(
                    "[{}] Asset's values are currently absolute, calculating rates of change first",
                    self.name
                );
                let roc = match &self.market_values.closing_prices(Side::Sell) {
                    Ok(vals) => rates_of_change(vals).collect::<Vec<_>>(),
                    Err(_) => return Err(()),
                };
                Ok(MuSigma {
                    mean: stats::mean(&roc),
                    std_dev: stats::sample_std_dev(&roc),
                })
            }
            (ValueType::Absolute, false) => {
                info!(
                    "[{}] Asset's values are currently absolute, calculating rates of change first",
                    self.name
                );
                let roc = match &self.market_values.closing_prices(Side::Buy) {
                    Ok(vals) => rates_of_change(vals).collect::<Vec<_>>(),
                    Err(_) => return Err(()),
                };
                Ok(MuSigma {
                    mean: stats::mean(&roc),
                    std_dev: stats::sample_std_dev(&roc),
                })
            }
            (ValueType::RateOfChange, true) => {
                let roc = match &self.market_values.closing_prices(Side::Sell) {
                    Ok(vals) => rates_of_change(vals).collect::<Vec<_>>(),
                    Err(_) => return Err(()),
                };

                Ok(MuSigma {
                    mean: stats::mean(&roc),
                    std_dev: stats::sample_std_dev(&roc),
                })
            }
            (ValueType::RateOfChange, false) => {
                let roc = match &self.market_values.closing_prices(Side::Buy) {
                    Ok(vals) => rates_of_change(vals).collect::<Vec<_>>(),
                    Err(_) => return Err(()),
                };

                Ok(MuSigma {
                    mean: stats::mean(&roc),
                    std_dev: stats::sample_std_dev(&roc),
                })
            }
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

#[cfg(feature = "std")]
impl ValueAtRisk for PortfolioAsset {
    fn value_at_risk_pct(&self, confidence: f64) -> Result<f64, ()> {
        crate::risk::var::varcovar::value_at_risk_percent(self, confidence)
    }

    fn value_at_risk(&self, confidence: f64, initial_investment: Option<f64>) -> Result<f64, ()> {
        match (
            self.mean_and_std_dev(),
            initial_investment,
            self.value_at_position_open,
        ) {
            (Err(_), _, _) => Err(()),
            (Ok(MuSigma { mean, std_dev }), Some(iv), _) => Ok(
                value_at_risk_from_initial_investment(confidence, mean, std_dev, iv),
            ),
            (Ok(MuSigma { mean, std_dev }), None, Some(vapo)) => {
                Ok(value_at_risk_from_initial_investment(
                    confidence,
                    mean,
                    std_dev,
                    vapo * self.quantity,
                ))
            }
            (_, _, None) => Err(()),
        }
    }
}

impl Volatility for PortfolioAsset {
    fn daily_volatility(&self) -> f64 {
        let vals = self.market_values.price_ranges_by(TimeSpan::Daily);

        if let Ok(v) = vals {
            v.iter()
                .map(|x| x.midpoint())
                .collect::<Vec<f64>>()
                .as_slice()
                .daily_volatility()
        } else {
            f64::NAN
        }
    }
}
