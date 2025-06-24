use crate::risk::var::ValueAtRisk;
use crate::util::roc::rates_of_change;
use num::traits::float::FloatCore;
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use alloc::vec::Vec;

// https://www.simtrade.fr/blog_simtrade/historical-method-var-calculation/

pub fn value_at_risk_percent(values: &[f64], confidence: f64) -> f64 {
    let mut roc = rates_of_change(values).collect::<Vec<_>>();

    roc.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let threshold = (confidence * roc.len() as f64).floor() as usize;

    roc[threshold]
}

#[cfg(feature = "rayon")]
pub fn par_value_at_risk_percent(values: &[f64], confidence: f64) -> f64 {
    let mut roc = rates_of_change(values).collect::<Vec<_>>();

    roc.par_sort_by(|x, y| x.partial_cmp(y).unwrap());

    let threshold = (confidence * roc.len() as f64).floor() as usize;

    roc[threshold]
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Historical {
    values: Vec<f64>,
}

impl Historical {
    pub fn new(values: &[f64]) -> Historical {
        Historical {
            values: Vec::from(values),
        }
    }
}

impl ValueAtRisk for Historical {
    fn value_at_risk_pct(&self, confidence: f64) -> Result<f64, ()> {
        Ok(value_at_risk_percent(&self.values, confidence))
    }

    fn value_at_risk(&self, _confidence: f64, _initial_investment: Option<f64>) -> Result<f64, ()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_test() {
        let result = value_at_risk_percent(&[1f64, 2f64, 4f64, 5f64], 0.01f64);
        assert_eq!(result, 0.25f64);
    }
}
