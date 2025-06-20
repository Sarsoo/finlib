use crate::derivatives::swaps::Swap;
use crate::price::enums::Side;
use crate::price::payoff::Payoff;
use pyo3::prelude::*;

#[pymethods]
impl Swap {
    #[new]
    pub fn init(fixed_rate: f64, fixed_side: Side, premium: f64) -> Self {
        Self::from(fixed_rate, fixed_side, premium)
    }

    #[pyo3(name = "payoff")]
    pub fn payoff_py(&self, underlying: f64) -> f64 {
        self.payoff(underlying)
    }

    #[pyo3(name = "payoff_from_multiple")]
    pub fn payoff_from_multiple_py(&self, underlying: Vec<f64>) -> f64 {
        self.payoff(underlying)
    }
}
