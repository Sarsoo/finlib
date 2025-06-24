//! Calculate Value at Risk using either the [`historical`] or parametric [`varcovar`] methods for an asset or portfolio

pub mod historical;
#[cfg(feature = "std")]
pub mod varcovar;
use num::traits::real::Real;

#[cfg(feature = "py")]
use pyo3::prelude::*;

pub trait ValueAtRisk {
    fn value_at_risk_pct(&self, confidence: f64) -> Result<f64, ()>;
    fn value_at_risk(&self, confidence: f64, initial_investment: Option<f64>) -> Result<f64, ()>;
    fn value_at_risk_after_time(
        &self,
        confidence: f64,
        initial_investment: Option<f64>,
        at: isize,
    ) -> Result<f64, ()> {
        Ok(scale_value_at_risk(
            self.value_at_risk(confidence, initial_investment)?,
            at,
        ))
    }
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn scale_value_at_risk(initial_value: f64, time_cycles: isize) -> f64 {
    initial_value * f64::sqrt(time_cycles as f64)
}
