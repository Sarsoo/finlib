use crate::derivatives::options::OptionContract;
use crate::derivatives::options::OptionStyle::European;
use crate::derivatives::options::OptionType::Call;
use crate::portfolio::strategy::strategy::Strategy;
use crate::portfolio::strategy::IStrategy;
use crate::price::enums::Side::{Buy, Sell};
use crate::price::payoff::Profit;

#[test]
fn basic_strategy() {
    let mut strat = Strategy::new();

    strat.add_component(OptionContract::from(Call, European, Buy, 1000., 10.));

    assert_eq!(strat.profit(1100.), 90.);
}

#[test]
fn basic_short_strategy() {
    let mut strat = Strategy::new();

    strat.add_component(OptionContract::from(Call, European, Sell, 1000., 10.));

    assert_eq!(strat.profit(1100.), -90.);
}
