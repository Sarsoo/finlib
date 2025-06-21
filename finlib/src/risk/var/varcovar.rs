use crate::stats;
use crate::util::roc::rates_of_change;
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::risk::var::ValueAtRisk;
use statrs::distribution::{ContinuousCDF, Normal};
// https://medium.com/@serdarilarslan/value-at-risk-var-and-its-implementation-in-python-5c9150f73b0e

pub fn value_at_risk_percent(values: &[f64], confidence: f64) -> f64 {
    let roc = rates_of_change(values).collect::<Vec<_>>();

    let mean = stats::mean(&roc);
    let std_dev = stats::sample_std_dev(&roc);

    let n = Normal::new(mean, std_dev).unwrap();

    n.inverse_cdf(confidence)
}

pub fn investment_value_at_risk(
    confidence: f64,
    investment_mean: f64,
    investment_std_dev: f64,
) -> f64 {
    let n = Normal::new(investment_mean, investment_std_dev).unwrap();

    n.inverse_cdf(confidence)
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VarianceCovariance {
    values: Vec<f64>,
}

impl VarianceCovariance {
    pub fn new(values: &[f64]) -> VarianceCovariance {
        VarianceCovariance {
            values: Vec::from(values),
        }
    }
}

impl ValueAtRisk for VarianceCovariance {
    fn value_at_risk_pct(&self, confidence: f64) -> f64 {
        value_at_risk_percent(&self.values, confidence)
    }
}

#[cfg(test)]
mod tests {
    use crate::portfolio::Portfolio;
    use crate::portfolio::PortfolioAsset;

    #[test]
    fn var_test() {
        let assets = vec![
            PortfolioAsset::new(
                // 0.3,
                "awdad".to_string(),
                4.0,
                vec![2f64, 3f64, 4f64],
            ),
            PortfolioAsset::new(
                // 0.7,
                "awdad".to_string(),
                4.0,
                vec![1f64, 6f64, 8f64],
            ),
        ];

        let mut portfolio = Portfolio::from(assets);

        portfolio.value_at_risk_percent(0.1);
    }

    #[test]
    fn var_test_one_asset() {
        let assets = vec![PortfolioAsset::new(
            // 0.3,
            "awdad".to_string(),
            4.0,
            vec![2f64, 3f64, 4f64],
        )];

        let mut portfolio = Portfolio::from(assets);

        portfolio.value_at_risk_percent(0.1);
    }

    #[test]
    fn var_test_one_asset_investment() {
        let assets = vec![
            PortfolioAsset::new(
                // 1.,
                "awdad".to_string(),
                4.0,
                vec![10., 9., 8., 7.],
            ), // PortfolioAsset::new(1., "awdad".to_string(), vec![2.1, 2., 2.1, 1., 1.])
        ];

        let mut portfolio = Portfolio::from(assets);

        println!("{:?}", portfolio.value_at_risk(0.01, 1_000_000.));
        println!("{:?}", portfolio.value_at_risk(0.1, 1_000_000.));
        println!("{:?}", portfolio.value_at_risk(0.5, 1_000_000.));
    }
}
