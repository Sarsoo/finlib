use log::info;
use crate::stats;
use crate::util::roc::rates_of_change;

use crate::risk::portfolio::Portfolio;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
use statrs::distribution::{ContinuousCDF, Normal};
use crate::risk::forecast::{mean_investment, std_dev_investment};
// https://medium.com/@serdarilarslan/value-at-risk-var-and-its-implementation-in-python-5c9150f73b0e

pub fn value_at_risk_percent(values: &[f64], confidence: f64) -> f64 {
    let roc = rates_of_change(values).collect::<Vec<_>>();

    let mean = stats::mean(&roc);
    let std_dev = stats::sample_std_dev(&roc);

    let n = Normal::new(mean, std_dev).unwrap();

    n.inverse_cdf(confidence)
}

pub fn portfolio_value_at_risk_percent(portfolio: &mut Portfolio, confidence: f64) -> Option<f64> {
    match portfolio.get_mean_and_std() {
        None => None,
        Some((mean, std_dev)) => {
            let n = Normal::new(mean, std_dev).unwrap();
            Some(n.inverse_cdf(confidence))
        }
    }
}

pub fn portfolio_value_at_risk(portfolio: &mut Portfolio, confidence: f64, initial_investment: f64) -> Option<f64> {
    match portfolio.get_mean_and_std() {
        None => None,
        Some((mean, std_dev)) => {
            let investment_mean = mean_investment(mean, initial_investment);
            let investment_std_dev = std_dev_investment(std_dev, std_dev);

            info!("{:?}, {:?}", investment_mean, investment_std_dev);

            let investment_var = investment_value_at_risk(confidence, investment_mean, investment_std_dev);

            println!("{:?}", investment_var);

            Some(initial_investment - investment_var)
        }
    }
}

pub fn investment_value_at_risk(confidence: f64, investment_mean: f64, investment_std_dev: f64) -> f64 {
    let n = Normal::new(investment_mean, investment_std_dev).unwrap();

    n.inverse_cdf(confidence)
}

pub fn scale_value_at_risk(initial_value: f64, time_cycles: isize) -> f64 {
    initial_value * f64::sqrt(time_cycles as f64)
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

        portfolio_value_at_risk_percent(&mut portfolio, 0.1);

    }

    #[test]
    fn var_test_one_asset() {
        let assets = vec![
            PortfolioAsset::new(0.3, "awdad".to_string(), vec![2f64, 3f64, 4f64])
        ];

        let mut portfolio = Portfolio::from(assets);

        portfolio_value_at_risk_percent(&mut portfolio, 0.1);
    }

    #[test]
    fn var_test_one_asset_investment() {
        let assets = vec![
            PortfolioAsset::new(1., "awdad".to_string(), vec![10., 9., 8., 7.])
            // PortfolioAsset::new(1., "awdad".to_string(), vec![2.1, 2., 2.1, 1., 1.])
        ];

        let mut portfolio = Portfolio::from(assets);

        println!("{:?}", portfolio_value_at_risk(&mut portfolio, 0.01, 1_000_000.));
        println!("{:?}", portfolio_value_at_risk(&mut portfolio, 0.1, 1_000_000.));
        println!("{:?}", portfolio_value_at_risk(&mut portfolio, 0.5, 1_000_000.));
    }
}