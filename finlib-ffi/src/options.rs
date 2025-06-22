use finlib::derivatives::options::blackscholes::option_surface::{
    OptionSurfaceParameters, OptionsSurface,
};
use finlib::derivatives::options::blackscholes::OptionVariables;
use finlib::derivatives::options::{OptionContract, OptionType};
use finlib::price::enums::Side;
use finlib::price::payoff::{Payoff, Profit};
use std::ops::Range;

#[no_mangle]
pub unsafe extern "C" fn option_vars_from(
    option_type: OptionType,
    underlying_price: f64,
    strike_price: f64,
    volatility: f64,
    risk_free_interest_rate: f64,
    dividend: f64,
    time_to_expiration: f64,
) -> *mut OptionVariables {
    Box::into_raw(Box::new(
        OptionVariables::builder()
            .option_type(option_type)
            .underlying_price(underlying_price)
            .strike_price(strike_price)
            .volatility(volatility)
            .risk_free_interest_rate(risk_free_interest_rate)
            .dividend(dividend)
            .time_to_expiration(time_to_expiration)
            .build(),
    ))
}

#[no_mangle]
pub unsafe extern "C" fn option_vars_destroy(option: *mut OptionVariables) {
    if !option.is_null() {
        drop(Box::from_raw(option));
    }
}

#[no_mangle]
pub unsafe extern "C" fn option_surface_parameters_from(
    underlying_price_range_min: isize,
    underlying_price_range_max: isize,
    underlying_price_min: f64,
    underlying_price_max: f64,

    strike_price_range_min: isize,
    strike_price_range_max: isize,
    strike_price_min: f64,
    strike_price_max: f64,

    volatility_range_min: isize,
    volatility_range_max: isize,
    volatility_min: f64,
    volatility_max: f64,

    risk_free_interest_rate_range_min: isize,
    risk_free_interest_rate_range_max: isize,
    risk_free_interest_rate_min: f64,
    risk_free_interest_rate_max: f64,

    dividend_range_min: isize,
    dividend_range_max: isize,
    dividend_min: f64,
    dividend_max: f64,

    time_to_expiration_range_min: isize,
    time_to_expiration_range_max: isize,
    time_to_expiration_min: f64,
    time_to_expiration_max: f64,
) -> *mut OptionSurfaceParameters {
    Box::into_raw(Box::new(OptionSurfaceParameters::from(
        Range {
            start: underlying_price_range_min,
            end: underlying_price_range_max,
        },
        (underlying_price_min, underlying_price_max),
        Range {
            start: strike_price_range_min,
            end: strike_price_range_max,
        },
        (strike_price_min, strike_price_max),
        Range {
            start: volatility_range_min,
            end: volatility_range_max,
        },
        (volatility_min, volatility_max),
        Range {
            start: risk_free_interest_rate_range_min,
            end: risk_free_interest_rate_range_max,
        },
        (risk_free_interest_rate_min, risk_free_interest_rate_max),
        Range {
            start: dividend_range_min,
            end: dividend_range_max,
        },
        (dividend_min, dividend_max),
        Range {
            start: time_to_expiration_range_min,
            end: time_to_expiration_range_max,
        },
        (time_to_expiration_min, time_to_expiration_max),
    )))
}

#[no_mangle]
pub unsafe extern "C" fn option_surface_parameters_walk(
    option: *mut OptionSurfaceParameters,
) -> *mut OptionsSurface {
    let pars = (*option).clone();

    match pars.walk() {
        Ok(s) => Box::into_raw(Box::new(s)),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn option_surface_parameters_destroy(option: *mut OptionSurfaceParameters) {
    if !option.is_null() {
        drop(Box::from_raw(option));
    }
}

#[no_mangle]
pub unsafe extern "C" fn option_surface_generate(option: *mut OptionsSurface) {
    let _ = (&mut *option).generate();
}

#[no_mangle]
pub unsafe extern "C" fn option_surface_par_generate(option: *mut OptionsSurface) {
    let _ = (&mut *option).par_generate();
}

#[no_mangle]
pub unsafe extern "C" fn option_surface_destroy(option: *mut OptionsSurface) {
    if !option.is_null() {
        drop(Box::from_raw(option));
    }
}

#[no_mangle]
pub unsafe extern "C" fn option_contract_from(
    option_type: OptionType,
    side: Side,
    strike: f64,
    premium: f64,
) -> *mut OptionContract {
    Box::into_raw(Box::new(OptionContract::from(
        option_type,
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
