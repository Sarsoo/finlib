use pyo3::prelude::*;
use crate::options::blackscholes::{OptionVariables, OptionGreeks};
use crate::risk::portfolio::{Portfolio, PortfolioAsset};

#[pymethods]
impl OptionVariables {
    #[new]
    pub fn init(underlying_price: f64,
                strike_price: f64,
                volatility: f64,
                risk_free_interest_rate: f64,
                dividend: f64,
                time_to_expiration: f64) -> Self {
        OptionVariables::from(underlying_price,
                              strike_price,
                              volatility,
                              risk_free_interest_rate,
                              dividend,
                              time_to_expiration)
    }
}