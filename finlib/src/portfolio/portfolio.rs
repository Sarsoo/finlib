use crate::portfolio::PortfolioAsset;
use crate::price::payoff::Payoff;
use crate::risk::forecast::{mean_investment, std_dev_investment};
use crate::risk::var::varcovar::investment_value_at_risk;
use log::{debug, error};
use ndarray::prelude::*;
use ndarray_stats::CorrelationExt;
#[cfg(feature = "py")]
use pyo3::prelude::*;
use rayon::prelude::*;
use statrs::distribution::{ContinuousCDF, Normal};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Describes a Portfolio as a collection of [`PortfolioAsset`]s
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
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

    /// Do the proportions of the assets in the portfolio add up to 100%?
    // pub fn valid_weights(&self) -> bool {
    //     let mut weight = 1f64;
    //
    //     for asset in &self.assets {
    //         weight -= asset.portfolio_weight;
    //     }
    //
    //     f64::abs(weight) < 0.01
    // }

    pub fn is_valid(&self) -> bool {
        self.valid_sizes()
        // && self.valid_weights()
    }

    /// Format the asset values in the portfolio as a matrix such that statistical operations can be applied to it
    pub fn get_matrix(&self) -> Option<Array2<f64>> {
        if self.assets.is_empty() || !self.valid_sizes() {
            return None;
        }

        let column_count = self.assets.len();
        let row_count = self.assets[0].market_values.len();

        let matrix = Array2::from_shape_vec(
            (column_count, row_count),
            self.assets
                .iter()
                .map(|a| a.market_values.clone())
                .flatten()
                .collect::<Vec<f64>>(),
        )
        .unwrap();
        Some(matrix.into_owned())
    }

    /// Format the asset values in the portfolio as a matrix such that statistical operations can be applied to it
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

    /// Calculate the mean and the standard deviation of a portfolio, taking into account the relative weights and covariance of the portfolio's assets
    ///
    /// returns (mean, std_dev)
    pub fn get_mean_and_std(&mut self) -> Option<(f64, f64)> {
        if !self.valid_sizes() {
            error!(
                "Can't get portfolio mean and std dev because asset value counts arent't the same"
            );
            return None;
        }

        self.apply_rates_of_change();
        let m = self.get_matrix();
        if m.is_none() {
            error!("Couldn't format portfolio as matrix");
            return None;
        }
        let m = m.unwrap();

        let cov = m.cov(1.);
        if cov.is_err() {
            error!("Failed to calculate portfolio covariance");
            return None;
        }
        let cov = cov.unwrap();
        let mean_return = m.mean_axis(Axis(1));
        if mean_return.is_none() {
            error!("Failed to calculate portfolio mean");
            return None;
        }
        let mean_return = mean_return.unwrap();
        let asset_weights =
            Array::from_vec(self.get_asset_weight().collect::<Vec<f64>>()).to_owned();

        let porfolio_mean_return = mean_return.dot(&asset_weights);
        let portfolio_stddev = f64::sqrt(asset_weights.t().dot(&cov).dot(&asset_weights));

        Some((porfolio_mean_return, portfolio_stddev))
    }

    /// For a given confidence rate (0.01, 0.05, 0.10) and initial investment value, calculate the parametric value at risk
    ///
    /// https://www.interviewqs.com/blog/value-at-risk
    pub fn value_at_risk(&mut self, confidence: f64, initial_investment: f64) -> Option<f64> {
        match self.get_mean_and_std() {
            None => None,
            Some((mean, std_dev)) => {
                debug!(
                    "Portfolio percent movement mean[{}], std dev[{}]",
                    mean, std_dev
                );
                let investment_mean = mean_investment(mean, initial_investment);
                let investment_std_dev = std_dev_investment(std_dev, initial_investment);
                debug!(
                    "Investment[{}] mean[{}], std dev[{}]",
                    initial_investment, mean, std_dev
                );

                let investment_var =
                    investment_value_at_risk(confidence, investment_mean, investment_std_dev);

                debug!(
                    "Investment[{}] value at risk [{}]",
                    initial_investment, investment_var
                );

                Some(initial_investment - investment_var)
            }
        }
    }

    /// For a given confidence rate (0.01, 0.05, 0.10) calculate the percentage change in an investment
    ///
    /// https://www.interviewqs.com/blog/value-at-risk
    pub fn value_at_risk_percent(&mut self, confidence: f64) -> Option<f64> {
        match self.get_mean_and_std() {
            None => None,
            Some((mean, std_dev)) => {
                let n = Normal::new(mean, std_dev).unwrap();
                Some(n.inverse_cdf(confidence))
            }
        }
    }
}

impl Payoff<Option<f64>> for Portfolio {
    fn payoff(&self, underlying: Option<f64>) -> f64 {
        self.assets.iter().map(|x| x.payoff(underlying)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let m = Portfolio::from(assets).get_matrix().unwrap();
        println!("matrix 0; {:?}", m);

        let col = m.row(0);
        println!("column 0; {:?}", col);
        let cov = m.cov(1.);

        println!("cov 0; {:?}", cov);

        col.len();
    }
}
