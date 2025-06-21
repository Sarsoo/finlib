//! Calculate Value at Risk using either the [`historical`] or parametric [`varcovar`] methods for an asset or portfolio

pub mod historical;
pub mod varcovar;

#[cfg(feature = "py")]
use pyo3::prelude::*;

pub trait ValueAtRisk {
    fn value_at_risk_pct(&self, confidence: f64) -> f64;
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn scale_value_at_risk(initial_value: f64, time_cycles: isize) -> f64 {
    initial_value * f64::sqrt(time_cycles as f64)
}
