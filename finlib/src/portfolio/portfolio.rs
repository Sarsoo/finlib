use crate::portfolio::{PortfolioAsset, ValueType};
use crate::price::payoff::{Payoff, Profit};
#[cfg(feature = "std")]
use crate::risk::var::varcovar::value_at_risk_from_initial_investment;
use crate::risk::var::ValueAtRisk;
use crate::stats::{MuSigma, PopulationStats};
use log::error;
use ndarray::prelude::*;
#[cfg(feature = "std")]
use ndarray_stats::CorrelationExt;
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use alloc::vec::Vec;

/// Describes a Portfolio as a collection of [`PortfolioAsset`]s
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Portfolio {
    assets: Vec<PortfolioAsset>,
}

impl Portfolio {
    pub fn from(assets: Vec<PortfolioAsset>) -> Portfolio {
        Portfolio { assets }
    }

    pub fn add_asset(&mut self, asset: PortfolioAsset) {
        self.assets.push(asset);
    }

    pub fn size(&self) -> usize {
        self.assets.len()
    }

    pub fn profit_loss(&self) -> Option<f64> {
        let asset_pl: Vec<Option<f64>> = self.assets.iter().map(|x| x.profit_loss()).collect();

        if asset_pl.iter().any(|x| x.is_none()) {
            None
        } else {
            Some(asset_pl.iter().map(|x| x.unwrap()).sum())
        }
    }

    /// Return the proportions of a portfolio's assets
    ///
    /// In a properly formed Portfolio these will add up to 1.0
    pub fn get_asset_weight(&self) -> impl Iterator<Item = f64> + use<'_> {
        let total_weight: f64 = self.assets.iter().map(|x| x.quantity).sum();

        // self.assets.iter().map(|x| x.portfolio_weight)
        self.assets.iter().map(move |x| x.quantity / total_weight)
    }

    /// Convert a portfolio of assets with absolute values to the percentage change in values
    pub fn apply_rates_of_change(&mut self) {
        self.assets.iter_mut().for_each(|asset| {
            asset.apply_rates_of_change();
        });
    }

    #[deprecated(note = "a lot slower than the sequential method, sans par prefix")]
    #[cfg(feature = "rayon")]
    pub fn par_apply_rates_of_change(&mut self) {
        self.assets.par_iter_mut().for_each(|asset| {
            asset.apply_rates_of_change();
        });
    }

    /// Do all the assets in the portfolio have the same number of values (required to perform matrix operations)
    pub fn valid_sizes(&self) -> bool {
        let mut last_value_length: Option<usize> = None;

        for asset in &self.assets {
            match last_value_length {
                None => {
                    last_value_length = Some(asset.market_values.len());
                }
                Some(l) => {
                    if l != asset.market_values.len() {
                        return false;
                    }
                    last_value_length = Some(asset.market_values.len());
                }
            }
        }

        true
    }

    pub fn is_valid(&self) -> bool {
        self.valid_sizes()
    }

    /// Format the asset values in the portfolio as a matrix such that statistical operations can be applied to it
    pub fn get_matrix(&self, f: &dyn Fn(&PortfolioAsset) -> Vec<f64>) -> Result<Array2<f64>, ()> {
        if self.assets.is_empty() || !self.valid_sizes() {
            return Err(());
        }

        let values = self.assets.iter().map(|a| f(a)).collect::<Vec<Vec<f64>>>();

        let sizes_match = values.iter().map(|x| x.len()).all(|x| x == values[0].len());

        if !sizes_match {
            return Err(());
        }

        let column_count = self.assets.len();
        let row_count = values[0].len();

        let values = values.into_iter().flatten().collect::<Vec<f64>>();

        let matrix = Array2::from_shape_vec((column_count, row_count), values).unwrap();

        Ok(matrix.into_owned())
    }

    fn get_raw_values(asset: &PortfolioAsset) -> Vec<f64> {
        asset.market_values.clone()
    }

    pub fn get_raw_matrix(&self) -> Result<Array2<f64>, ()> {
        self.get_matrix(&Self::get_raw_values)
    }

    fn get_roc_values(asset: &PortfolioAsset) -> Vec<f64> {
        asset.get_rates_of_change()
    }

    pub fn get_roc_matrix(&self) -> Result<Array2<f64>, ()> {
        self.get_matrix(&Self::get_roc_values)
    }

    /// Format the asset values in the portfolio as a matrix such that statistical operations can be applied to it
    #[cfg(feature = "rayon")]
    pub fn par_get_matrix(&self) -> Option<Array2<f64>> {
        if self.assets.is_empty() || !self.valid_sizes() {
            return None;
        }

        let column_count = self.assets.len();
        let row_count = self.assets[0].market_values.len();

        let matrix = Array2::from_shape_vec(
            (column_count, row_count),
            self.assets
                .par_iter()
                .map(|a| a.market_values.clone())
                .flatten()
                .collect::<Vec<f64>>(),
        )
        .unwrap();
        Some(matrix.into_owned())
    }

    pub fn initial_investment(&self) -> Result<f64, ()> {
        if self
            .assets
            .iter()
            .all(|x| x.value_at_position_open.is_none())
        {
            error!("portfolio: invalid initial investment retrieved, all are 0");
            return Err(());
        }

        Ok(self
            .assets
            .iter()
            .map(|x| match x.value_at_position_open {
                None => 0.0,
                Some(iv) => iv * x.quantity,
            })
            .sum())
    }

    pub fn is_differential(&self) -> bool {
        !self
            .assets
            .iter()
            .any(|x| x.value_type == ValueType::Absolute)
    }
}

#[cfg(feature = "std")]
impl ValueAtRisk for Portfolio {
    /// For a given confidence rate (0.01, 0.05, 0.10) calculate the percentage change in an investment
    ///
    /// https://www.interviewqs.com/blog/value-at-risk
    fn value_at_risk_pct(&self, confidence: f64) -> Result<f64, ()> {
        crate::risk::var::varcovar::value_at_risk_percent(self, confidence)
    }

    /// For a given confidence rate (0.01, 0.05, 0.10) and initial investment value, calculate the parametric value at risk
    ///
    /// https://www.interviewqs.com/blog/value-at-risk
    fn value_at_risk(&self, confidence: f64, initial_investment: Option<f64>) -> Result<f64, ()> {
        match (self.mean_and_std_dev(), initial_investment) {
            (Err(_), _) => Err(()),
            (Ok(MuSigma { mean, std_dev }), Some(iv)) => Ok(value_at_risk_from_initial_investment(
                confidence, mean, std_dev, iv,
            )),
            (Ok(MuSigma { mean, std_dev }), None) => match self.initial_investment() {
                Ok(iv) => Ok(value_at_risk_from_initial_investment(
                    confidence, mean, std_dev, iv,
                )),
                Err(_) => Err(()),
            },
        }
    }
}

#[cfg(feature = "std")]
impl PopulationStats for Portfolio {
    /// Calculate the mean and the standard deviation of a portfolio, taking into account the relative weights and covariance of the portfolio's assets
    ///
    /// returns (mean, std_dev)
    fn mean_and_std_dev(&self) -> Result<MuSigma, ()> {
        if !self.valid_sizes() {
            error!(
                "Can't get portfolio mean and std dev because asset value counts aren't the same"
            );
            return Err(());
        }

        let m = self.get_roc_matrix();

        if m.is_err() {
            error!("Couldn't format portfolio as matrix");
            return Err(());
        }
        let m = m.unwrap();

        let cov = m.cov(1.);
        if cov.is_err() {
            error!("Failed to calculate portfolio covariance");
            return Err(());
        }
        let cov = cov.unwrap();
        assert_eq!(cov.shape()[0], self.assets.len());
        assert_eq!(cov.shape()[1], self.assets.len());
        let mean_return = m.mean_axis(Axis(1));
        if mean_return.is_none() {
            error!("Failed to calculate portfolio mean");
            return Err(());
        }
        let mean_return = mean_return.unwrap();
        let asset_weights =
            Array::from_vec(self.get_asset_weight().collect::<Vec<f64>>()).to_owned();

        let porfolio_mean_return = mean_return.dot(&asset_weights);
        let portfolio_stddev = f64::sqrt(asset_weights.t().dot(&cov).dot(&asset_weights));

        Ok(MuSigma {
            mean: porfolio_mean_return,
            std_dev: portfolio_stddev,
        })
    }
}

impl Payoff<Option<f64>> for Portfolio {
    fn payoff(&self, underlying: Option<f64>) -> f64 {
        self.assets.iter().map(|x| x.payoff(underlying)).sum()
    }
}

impl Profit<Option<f64>> for Portfolio {
    fn profit(&self, underlying: Option<f64>) -> f64 {
        self.payoff(underlying)
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;

    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn get_matrix() {
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

        let m = Portfolio::from(assets).get_raw_matrix().unwrap();
        println!("matrix 0; {:?}", m);

        let col = m.row(0);
        println!("column 0; {:?}", col);
        let cov = m.cov(1.);

        println!("cov 0; {:?}", cov);

        col.len();
    }

    #[test]
    fn mean_std_dev() {
        let assets = vec![
            PortfolioAsset::new("awdad".to_string(), 1.0, vec![0.5, 0.5, 0.5, 0.5]),
            PortfolioAsset::new("awdad".to_string(), 1.0, vec![0.5, 0.5, 0.5, 0.5]),
        ];

        let m = Portfolio::from(assets);

        let stats = m.mean_and_std_dev();

        assert!(stats.is_ok());
    }

    #[test]
    fn var_investment() {
        let assets = vec![
            PortfolioAsset::builder()
                .name("awdad".into())
                .quantity(4.0)
                .market_values(vec![2f64, 3f64, 4f64])
                .value_at_position_open(1.0)
                .value_type(ValueType::Absolute)
                .build(),
            PortfolioAsset::builder()
                .name("awdad".into())
                .quantity(4.0)
                .market_values(vec![1f64, 6f64, 8f64])
                .value_at_position_open(1.0)
                .value_type(ValueType::Absolute)
                .build(),
        ];

        let m = Portfolio::from(assets);

        assert!(m.value_at_risk(0.01, None).is_ok());
    }

    #[test]
    fn var_investment_error() {
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

        let m = Portfolio::from(assets);

        assert!(m.value_at_risk(0.01, None).is_err());
    }
}
