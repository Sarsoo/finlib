use finlib::derivatives::options::strategy::component::OptionStrategyComponent;
use finlib::derivatives::options::strategy::strategy::OptionStrategy;
use finlib::derivatives::options::strategy::{IOptionStrategy, IOptionStrategyComponent};
use finlib::derivatives::options::OptionType;
use finlib::price::enums::Side;
use finlib::price::payoff::{Payoff, Profit};

#[no_mangle]
pub unsafe extern "C" fn option_strategy_new() -> *mut OptionStrategy {
    Box::into_raw(Box::new(OptionStrategy::new()))
}

#[no_mangle]
pub unsafe extern "C" fn option_strategy_size(option: *mut OptionStrategy) -> usize {
    (&mut *option).size()
}

#[no_mangle]
pub unsafe extern "C" fn option_strategy_add_component(
    option: *mut OptionStrategy,
    component: *mut OptionStrategyComponent,
) {
    (&mut *option).add_component((*component).clone());
}

#[no_mangle]
pub unsafe extern "C" fn option_strategy_payoff(
    option: *mut OptionStrategy,
    underlying: f64,
) -> f64 {
    (&mut *option).payoff(underlying)
}

#[no_mangle]
pub unsafe extern "C" fn option_strategy_profit(
    option: *mut OptionStrategy,
    underlying: f64,
) -> f64 {
    (&mut *option).profit(underlying)
}

#[no_mangle]
pub unsafe extern "C" fn option_strategy_destroy(option: *mut OptionStrategy) {
    if !option.is_null() {
        drop(Box::from_raw(option));
    }
}

#[no_mangle]
pub unsafe extern "C" fn option_strategy_component_from(
    option_type: OptionType,
    side: Side,
    strike: f64,
    premium: f64,
) -> *mut OptionStrategyComponent {
    Box::into_raw(Box::new(OptionStrategyComponent::from(
        option_type,
        side,
        strike,
        premium,
    )))
}

#[no_mangle]
pub unsafe extern "C" fn option_strategy_component_payoff(
    option: *mut OptionStrategyComponent,
    underlying: f64,
) -> f64 {
    (&mut *option).payoff(underlying)
}

#[no_mangle]
pub unsafe extern "C" fn option_strategy_component_profit(
    option: *mut OptionStrategyComponent,
    underlying: f64,
) -> f64 {
    (&mut *option).profit(underlying)
}

#[no_mangle]
pub unsafe extern "C" fn option_strategy_component_will_be_exercised(
    option: *mut OptionStrategyComponent,
    underlying: f64,
) -> bool {
    (&mut *option).will_be_exercised(underlying)
}

#[no_mangle]
pub unsafe extern "C" fn option_strategy_component_destroy(option: *mut OptionStrategyComponent) {
    if !option.is_null() {
        drop(Box::from_raw(option));
    }
}
