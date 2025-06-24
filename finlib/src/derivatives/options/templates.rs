use crate::derivatives::options::OptionType::{Call, Put};
use crate::price::enums::Side::{Buy, Sell};

use alloc::vec;
use alloc::vec::Vec;

use crate::derivatives::options::OptionContract;
use crate::derivatives::options::OptionStyle::European;
#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg_attr(feature = "py", pyfunction)]
pub fn bull_call_spread(
    floor_strike: f64,
    floor_premium: f64,
    ceiling_strike: f64,
    ceiling_premium: f64,
) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Call, European, Buy, floor_strike, floor_premium),
        OptionContract::from(Call, European, Sell, ceiling_strike, ceiling_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn bear_put_spread(
    floor_strike: f64,
    floor_premium: f64,
    ceiling_strike: f64,
    ceiling_premium: f64,
) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Put, European, Buy, ceiling_strike, ceiling_premium),
        OptionContract::from(Put, European, Sell, floor_strike, floor_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn collar(
    put_strike: f64,
    put_premium: f64,
    call_strike: f64,
    call_premium: f64,
) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Put, European, Buy, put_strike, put_premium),
        OptionContract::from(Call, European, Sell, call_strike, call_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn long_straddle(strike: f64, put_premium: f64, call_premium: f64) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Put, European, Buy, strike, put_premium),
        OptionContract::from(Call, European, Buy, strike, call_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn long_strangle(
    put_strike: f64,
    put_premium: f64,
    call_strike: f64,
    call_premium: f64,
) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Put, European, Buy, put_strike, put_premium),
        OptionContract::from(Call, European, Buy, call_strike, call_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn long_call_butterfly_spread(
    itm_strike: f64,
    itm_premium: f64,
    atm_strike: f64,
    atm_premium: f64,
    otm_strike: f64,
    otm_premium: f64,
) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Call, European, Buy, itm_strike, itm_premium),
        OptionContract::from(Call, European, Sell, atm_strike, atm_premium),
        OptionContract::from(Call, European, Sell, atm_strike, atm_premium),
        OptionContract::from(Call, European, Buy, otm_strike, otm_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn short_call_butterfly_spread(
    itm_strike: f64,
    itm_premium: f64,
    atm_strike: f64,
    atm_premium: f64,
    otm_strike: f64,
    otm_premium: f64,
) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Call, European, Sell, itm_strike, itm_premium),
        OptionContract::from(Call, European, Buy, atm_strike, atm_premium),
        OptionContract::from(Call, European, Buy, atm_strike, atm_premium),
        OptionContract::from(Call, European, Sell, otm_strike, otm_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn long_put_butterfly_spread(
    itm_strike: f64,
    itm_premium: f64,
    atm_strike: f64,
    atm_premium: f64,
    otm_strike: f64,
    otm_premium: f64,
) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Put, European, Buy, itm_strike, itm_premium),
        OptionContract::from(Put, European, Sell, atm_strike, atm_premium),
        OptionContract::from(Put, European, Sell, atm_strike, atm_premium),
        OptionContract::from(Put, European, Buy, otm_strike, otm_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn short_put_butterfly_spread(
    itm_strike: f64,
    itm_premium: f64,
    atm_strike: f64,
    atm_premium: f64,
    otm_strike: f64,
    otm_premium: f64,
) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Put, European, Sell, itm_strike, itm_premium),
        OptionContract::from(Put, European, Buy, atm_strike, atm_premium),
        OptionContract::from(Put, European, Buy, atm_strike, atm_premium),
        OptionContract::from(Put, European, Sell, otm_strike, otm_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn iron_butterfly_spread(
    atm_strike: f64,
    atm_premium: f64,
    otm_strike: f64,
    otm_premium: f64,
) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Call, European, Buy, otm_strike, otm_premium),
        OptionContract::from(Call, European, Sell, atm_strike, atm_premium),
        OptionContract::from(Put, European, Sell, atm_strike, atm_premium),
        OptionContract::from(Put, European, Buy, otm_strike, otm_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn reverse_iron_butterfly_spread(
    atm_strike: f64,
    atm_premium: f64,
    otm_strike: f64,
    otm_premium: f64,
) -> Vec<OptionContract> {
    vec![
        OptionContract::from(Call, European, Buy, atm_strike, atm_premium),
        OptionContract::from(Put, European, Buy, atm_strike, atm_premium),
        OptionContract::from(Call, European, Sell, otm_strike, otm_premium),
        OptionContract::from(Put, European, Sell, otm_strike, otm_premium),
    ]
}
