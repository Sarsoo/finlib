use crate::derivatives::options::strategy::component::OptionStrategyComponent;
use crate::derivatives::options::OptionType::{Call, Put};
use crate::price::enums::Side::{Buy, Sell};

#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg_attr(feature = "py", pyfunction)]
pub fn bull_call_spread(
    floor_strike: f64,
    floor_premium: f64,
    ceiling_strike: f64,
    ceiling_premium: f64,
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Call, Buy, floor_strike, floor_premium),
        OptionStrategyComponent::from(Call, Sell, ceiling_strike, ceiling_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn bear_put_spread(
    floor_strike: f64,
    floor_premium: f64,
    ceiling_strike: f64,
    ceiling_premium: f64,
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Put, Buy, ceiling_strike, ceiling_premium),
        OptionStrategyComponent::from(Put, Sell, floor_strike, floor_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn collar(
    put_strike: f64,
    put_premium: f64,
    call_strike: f64,
    call_premium: f64,
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Put, Buy, put_strike, put_premium),
        OptionStrategyComponent::from(Call, Sell, call_strike, call_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn long_straddle(
    strike: f64,
    put_premium: f64,
    call_premium: f64,
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Put, Buy, strike, put_premium),
        OptionStrategyComponent::from(Call, Buy, strike, call_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn long_strangle(
    put_strike: f64,
    put_premium: f64,
    call_strike: f64,
    call_premium: f64,
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Put, Buy, put_strike, put_premium),
        OptionStrategyComponent::from(Call, Buy, call_strike, call_premium),
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
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Call, Buy, itm_strike, itm_premium),
        OptionStrategyComponent::from(Call, Sell, atm_strike, atm_premium),
        OptionStrategyComponent::from(Call, Sell, atm_strike, atm_premium),
        OptionStrategyComponent::from(Call, Buy, otm_strike, otm_premium),
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
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Call, Sell, itm_strike, itm_premium),
        OptionStrategyComponent::from(Call, Buy, atm_strike, atm_premium),
        OptionStrategyComponent::from(Call, Buy, atm_strike, atm_premium),
        OptionStrategyComponent::from(Call, Sell, otm_strike, otm_premium),
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
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Put, Buy, itm_strike, itm_premium),
        OptionStrategyComponent::from(Put, Sell, atm_strike, atm_premium),
        OptionStrategyComponent::from(Put, Sell, atm_strike, atm_premium),
        OptionStrategyComponent::from(Put, Buy, otm_strike, otm_premium),
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
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Put, Sell, itm_strike, itm_premium),
        OptionStrategyComponent::from(Put, Buy, atm_strike, atm_premium),
        OptionStrategyComponent::from(Put, Buy, atm_strike, atm_premium),
        OptionStrategyComponent::from(Put, Sell, otm_strike, otm_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn iron_butterfly_spread(
    atm_strike: f64,
    atm_premium: f64,
    otm_strike: f64,
    otm_premium: f64,
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Call, Buy, otm_strike, otm_premium),
        OptionStrategyComponent::from(Call, Sell, atm_strike, atm_premium),
        OptionStrategyComponent::from(Put, Sell, atm_strike, atm_premium),
        OptionStrategyComponent::from(Put, Buy, otm_strike, otm_premium),
    ]
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn reverse_iron_butterfly_spread(
    atm_strike: f64,
    atm_premium: f64,
    otm_strike: f64,
    otm_premium: f64,
) -> Vec<OptionStrategyComponent> {
    vec![
        OptionStrategyComponent::from(Call, Buy, atm_strike, atm_premium),
        OptionStrategyComponent::from(Put, Buy, atm_strike, atm_premium),
        OptionStrategyComponent::from(Call, Sell, otm_strike, otm_premium),
        OptionStrategyComponent::from(Put, Sell, otm_strike, otm_premium),
    ]
}
