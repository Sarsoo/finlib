use crate::stats;
use crate::util::roc::rates_of_change;
use ndarray_stats::CorrelationExt;

use crate::risk::portfolio::Portfolio;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
use statrs::distribution::{ContinuousCDF, Normal};
// https://medium.com/@serdarilarslan/value-at-risk-var-and-its-implementation-in-python-5c9150f73b0e

pub fn value_at_risk(values: &[f64], confidence: f64) -> f64 {
    let roc = rates_of_change(values).collect::<Vec<_>>();

    let mean = stats::mean(&roc);
    let std_dev = stats::sample_std_dev(&roc);

    let n = Normal::new(0.0, 1.0).unwrap();

    mean + std_dev * n.inverse_cdf(confidence)
}

pub fn portfolio_value_at_risk(portfolio: &mut Portfolio, confidence: f64) -> Option<f64> {
    match portfolio.get_mean_and_std() {
        None => None,
        Some((mean, std_dev)) => {
            let n = Normal::new(0.0, 1.0).unwrap();
            Some(mean + std_dev * n.inverse_cdf(confidence))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::risk::portfolio::PortfolioAsset;

    #[test]
    fn var_test() {
        let assets = vec![
            PortfolioAsset::new(0.3, "awdad".to_string(), vec![2f64, 3f64, 4f64]),
            PortfolioAsset::new(0.7, "awdad".to_string(), vec![1f64, 6f64, 8f64]),
        ];

        let mut portfolio = Portfolio::from(assets);

        portfolio_value_at_risk(&mut portfolio, 0.1);

    }

    #[test]
    fn var_test_one_asset() {
        let assets = vec![
            PortfolioAsset::new(0.3, "awdad".to_string(), vec![2f64, 3f64, 4f64])
        ];

        let mut portfolio = Portfolio::from(assets);

        portfolio_value_at_risk(&mut portfolio, 0.1);
    }
}