use num::{Float, NumCast};

pub fn investment_mean_from_portfolio<T: Float>(
    portfolio_mean_change: T,
    initial_investment: T,
) -> T {
    let one: T = NumCast::from(1).unwrap();
    (one + portfolio_mean_change) * initial_investment
}

pub fn investment_std_dev_from_portfolio<T: Float>(
    portfolio_change_stddev: T,
    initial_investment: T,
) -> T {
    portfolio_change_stddev * initial_investment
}
