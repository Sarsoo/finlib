use alloc::boxed::Box;
use finlib::derivatives::options::OptionContract;
use finlib::derivatives::swaps::Swap;
use finlib::portfolio::strategy::strategy::Strategy;
use finlib::portfolio::strategy::IStrategy;
use finlib::price::payoff::{Payoff, Profit};

#[no_mangle]
pub unsafe extern "C" fn strategy_new() -> *mut Strategy {
    Box::into_raw(Box::new(Strategy::new()))
}

#[no_mangle]
pub unsafe extern "C" fn strategy_size(option: *mut Strategy) -> usize {
    (&mut *option).size()
}

#[no_mangle]
pub unsafe extern "C" fn strategy_add_option_component(
    option: *mut Strategy,
    component: *mut OptionContract,
) {
    (&mut *option).add_component((*component).clone());
}

#[no_mangle]
pub unsafe extern "C" fn strategy_add_swap_component(option: *mut Strategy, component: *mut Swap) {
    (&mut *option).add_component((*component).clone());
}

#[no_mangle]
pub unsafe extern "C" fn strategy_payoff(option: *mut Strategy, underlying: f64) -> f64 {
    (&mut *option).payoff(underlying)
}

#[no_mangle]
pub unsafe extern "C" fn strategy_profit(option: *mut Strategy, underlying: f64) -> f64 {
    (&mut *option).profit(underlying)
}

#[no_mangle]
pub unsafe extern "C" fn strategy_destroy(option: *mut Strategy) {
    if !option.is_null() {
        drop(Box::from_raw(option));
    }
}
