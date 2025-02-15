use statrs::distribution::{ContinuousCDF, Normal};

pub fn mean_investment(portfolio_mean_change: f64, initial_investment: f64) -> f64 {
    (1. + portfolio_mean_change) * initial_investment
}

pub fn std_dev_investment(portfolio_change_stddev: f64, initial_investment: f64) -> f64 {
    portfolio_change_stddev * initial_investment
}