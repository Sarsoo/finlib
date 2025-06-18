use num::{Float, NumCast};

pub fn mean_investment<T: Float>(portfolio_mean_change: T, initial_investment: T) -> T {
    let one: T = NumCast::from(1).unwrap();
    (one + portfolio_mean_change) * initial_investment
}

pub fn std_dev_investment<T: Float>(portfolio_change_stddev: T, initial_investment: T) -> T {
    portfolio_change_stddev * initial_investment
}