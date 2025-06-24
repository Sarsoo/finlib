use alloc::boxed::Box;
use finlib::derivatives::options::{OptionContract, OptionStyle, OptionType};
use finlib::price::enums::Side;
use finlib::price::payoff::{Payoff, Profit};

#[no_mangle]
pub unsafe extern "C" fn option_contract_from(
    option_type: OptionType,
    option_style: OptionStyle,
    side: Side,
    strike: f64,
    premium: f64,
) -> *mut OptionContract {
    Box::into_raw(Box::new(OptionContract::from(
        option_type,
        option_style,
        side,
        strike,
        premium,
    )))
}

#[no_mangle]
pub unsafe extern "C" fn option_contract_payoff(
    option: *mut OptionContract,
    underlying: f64,
) -> f64 {
    (&mut *option).payoff(underlying)
}

#[no_mangle]
pub unsafe extern "C" fn option_contract_profit(
    option: *mut OptionContract,
    underlying: f64,
) -> f64 {
    (&mut *option).profit(underlying)
}

#[no_mangle]
pub unsafe extern "C" fn option_contract_will_be_exercised(
    option: *mut OptionContract,
    underlying: f64,
) -> bool {
    (&mut *option).will_be_exercised(underlying)
}

#[no_mangle]
pub unsafe extern "C" fn option_contract_destroy(option: *mut OptionContract) {
    if !option.is_null() {
        drop(Box::from_raw(option));
    }
}
