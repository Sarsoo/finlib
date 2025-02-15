use ndarray::prelude::*;
use ndarray_stats::CorrelationExt;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "py")]
use pyo3::prelude::*;
use crate::risk::var::varcovar::{portfolio_value_at_risk, portfolio_value_at_risk_percent};
use crate::stats;
use crate::util::roc::rates_of_change;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass)]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Clone)]
pub struct Portfolio {
    assets: Vec<PortfolioAsset>
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ValueType {
    Absolute,
    RateOfChange
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass)]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Clone)]
pub struct PortfolioAsset {
    portfolio_weight: f64,
    name: String,
    values: Vec<f64>,
    value_type: ValueType
}

impl PortfolioAsset {
    pub fn new(portfolio_weight: f64, name: String, values: Vec<f64>) -> PortfolioAsset {
        PortfolioAsset {
            portfolio_weight, name, values, value_type: ValueType::Absolute
        }
    }

    pub fn apply_rates_of_change(&mut self) {
        match self.value_type {
            ValueType::Absolute => {
                self.values = rates_of_change(&self.values).collect();
                self.value_type = ValueType::RateOfChange;
            }
            _ => {}
        }
    }

    pub fn get_mean_and_std(&self) -> Option<(f64, f64)> {
        match self.value_type {
            ValueType::Absolute => {
                let roc = rates_of_change(&self.values).collect::<Vec<f64>>();
                Some((stats::mean(&roc), stats::sample_std_dev(&roc)))
            }
            ValueType::RateOfChange => {
                Some((stats::mean(&self.values), stats::sample_std_dev(&self.values)))
            }
        }
    }
}

impl Portfolio {
    pub fn from(assets: Vec<PortfolioAsset>) -> Portfolio {
        Portfolio {
            assets
        }
    }

    pub fn get_asset_weight(&self) -> impl Iterator<Item=f64> + use<'_> {
        self.assets
            .iter()
            .map(|x| x.portfolio_weight)
    }

    pub fn apply_rates_of_change(&mut self) {
        for asset in self.assets.iter_mut() {
            asset.apply_rates_of_change();
        }
    }

    pub fn valid_sizes(&self) -> bool {
        let mut last_value_length: Option<usize> = None;

        for asset in &self.assets {
            match last_value_length {
                None => {
                    last_value_length = Some(asset.values.len());
                }
                Some(l) => {
                    if l != asset.values.len() {
                        return false;
                    }
                    last_value_length = Some(asset.values.len());
                }
            }
        }

        true
    }

    pub fn valid_weights(&self) -> bool {
        let mut weight = 1 as f64;

        for asset in &self.assets {
            weight -= asset.portfolio_weight;
        }

        f64::abs(weight) < 0.01
    }

    pub fn is_valid(&self) -> bool {
        self.valid_sizes() && self.valid_weights()
    }

    pub fn get_matrix(&self) -> Option<Array2<f64>> {
        if self.assets.is_empty() || !self.valid_sizes() {
            return None;
        }

        let column_count = self.assets.len();
        let row_count = self.assets[0].values.len();

        let matrix = Array2::from_shape_vec((column_count, row_count),
            self.assets
                .iter()
                .map(|a| a.values.clone())
                .flatten()
                .collect::<Vec<f64>>()
        ).unwrap();

        Some(matrix.into_owned())
    }

    pub fn get_mean_and_std(&mut self) -> Option<(f64, f64)> {
        if !self.valid_sizes() {
            return None;
        }

        self.apply_rates_of_change();
        let m = self.get_matrix();
        if m.is_none() {
            return None;
        }
        let m = m.unwrap();

        let cov = m.cov(1.);
        if cov.is_err() {
            return None;
        }
        let cov = cov.unwrap();
        let mean_return = m.mean_axis(Axis(1));
        if mean_return.is_none() {
            return None;
        }
        let mean_return = mean_return.unwrap();
        let asset_weights = Array::from_vec(
            self.get_asset_weight().collect::<Vec<f64>>()
        ).to_owned();

        let porfolio_mean_return = mean_return.dot(&asset_weights);
        let portfolio_stddev = f64::sqrt(asset_weights.t().dot(&cov).dot(&asset_weights));

        Some((porfolio_mean_return, portfolio_stddev))
    }

    pub fn value_at_risk(&mut self, confidence: f64, initial_investment: f64) -> Option<f64> {
        portfolio_value_at_risk(self, confidence, initial_investment)
    }

    pub fn value_at_risk_percent(&mut self, confidence: f64) -> Option<f64> {
        portfolio_value_at_risk_percent(self, confidence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_test() {
        let assets = vec![
            PortfolioAsset::new(0.3, "awdad".to_string(), vec![2f64, 3f64, 4f64]),
            PortfolioAsset::new(0.7, "awdad".to_string(), vec![1f64, 6f64, 8f64]),
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