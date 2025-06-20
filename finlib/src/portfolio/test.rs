use crate::derivatives::options::strategy::component::OptionStrategyComponent;
use crate::derivatives::options::strategy::strategy::OptionStrategy;
use crate::derivatives::options::strategy::IOptionStrategy;
use crate::derivatives::options::OptionType::Call;
use crate::price::enums::Side::{Buy, Sell};
use crate::price::payoff::Profit;

#[test]
fn basic_strategy() {
    let mut strat = OptionStrategy::new();

    strat.add_component(OptionStrategyComponent::from(Call, Buy, 1000., 10.));

    assert_eq!(strat.profit(1100.), 90.);
}

#[test]
fn basic_short_strategy() {
    let mut strat = OptionStrategy::new();

    strat.add_component(OptionStrategyComponent::from(Call, Sell, 1000., 10.));

    assert_eq!(strat.profit(1100.), -90.);
}
