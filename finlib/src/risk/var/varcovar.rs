use crate::stats;
use crate::util::roc::rates_of_change;
use log::debug;
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use alloc::vec::Vec;

use crate::risk::forecast::{investment_mean_from_portfolio, investment_std_dev_from_portfolio};
use crate::risk::var::ValueAtRisk;
use crate::stats::{inverse_cdf_value, MuSigma, PopulationStats};
// https://medium.com/@serdarilarslan/value-at-risk-var-and-its-implementation-in-python-5c9150f73b0e

pub fn value_at_risk_percent(sample: &impl PopulationStats, confidence: f64) -> Result<f64, ()> {
    match sample.mean_and_std_dev() {
        Err(_) => Err(()),
        Ok(MuSigma { mean, std_dev }) => Ok(inverse_cdf_value(confidence, mean, std_dev)),
    }
}

pub fn value_at_risk_percent_1d(values: &[f64], confidence: f64) -> f64 {
    let roc = rates_of_change(values).collect::<Vec<_>>();

    let mean = stats::mean(&roc);
    let std_dev = stats::sample_std_dev(&roc);

    inverse_cdf_value(confidence, mean, std_dev)
}

pub fn value_at_risk_from_initial_investment(
    confidence: f64,
    mean: f64,
    std_dev: f64,
    initial_investment: f64,
) -> f64 {
    debug!(
        "Portfolio percent movement mean[{}], std dev[{}]",
        mean, std_dev
    );
    let investment_mean = investment_mean_from_portfolio(mean, initial_investment);
    let investment_std_dev = investment_std_dev_from_portfolio(std_dev, initial_investment);
    debug!(
        "Investment[{}] mean[{}], std dev[{}]",
        initial_investment, mean, std_dev
    );

    let investment_var = inverse_cdf_value(confidence, investment_mean, investment_std_dev);

    debug!(
        "Investment[{}] value at risk [{}]",
        initial_investment, investment_var
    );

    initial_investment - investment_var
}

pub fn value_at_risk_from_initial_investment_1d(
    values: &[f64],
    confidence: f64,
    initial_investment: f64,
) -> f64 {
    let roc = rates_of_change(values).collect::<Vec<_>>();

    let mean = stats::mean(&roc);
    let std_dev = stats::sample_std_dev(&roc);

    value_at_risk_from_initial_investment(confidence, mean, std_dev, initial_investment)
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
    fn value_at_risk_pct(&self, confidence: f64) -> Result<f64, ()> {
        Ok(value_at_risk_percent_1d(&self.values, confidence))
    }

    fn value_at_risk(&self, confidence: f64, initial_investment: Option<f64>) -> Result<f64, ()> {
        match initial_investment {
            None => Err(()),
            Some(iv) => Ok(value_at_risk_from_initial_investment_1d(
                &self.values,
                confidence,
                iv,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::portfolio::Portfolio;
    use crate::portfolio::PortfolioAsset;
    use crate::risk::var::ValueAtRisk;

    use alloc::string::ToString;
    use alloc::vec;

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

        let portfolio = Portfolio::from(assets);

        let _ = portfolio.value_at_risk_pct(0.1);
    }

    #[test]
    fn var_test_one_asset() {
        let assets = vec![PortfolioAsset::new(
            // 0.3,
            "awdad".to_string(),
            4.0,
            vec![2f64, 3f64, 4f64],
        )];

        let portfolio = Portfolio::from(assets);

        let _ = portfolio.value_at_risk_pct(0.1);
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

        let portfolio = Portfolio::from(assets);

        println!("{:?}", portfolio.value_at_risk(0.01, Some(1_000_000.)));
        println!("{:?}", portfolio.value_at_risk(0.1, Some(1_000_000.)));
        println!("{:?}", portfolio.value_at_risk(0.5, Some(1_000_000.)));
    }
}
