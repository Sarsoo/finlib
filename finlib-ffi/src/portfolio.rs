use crate::{NullableFloat, Tuple};
use finlib::portfolio::{Portfolio, PortfolioAsset};
use finlib::price::payoff::{Payoff, Profit};
use finlib::risk::var::ValueAtRisk;
use finlib::stats::{MuSigma, PopulationStats};
use std::{ptr, slice};

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_new(
    // portfolio_weight: f64,
    name: *const u8,
    name_len: i32,
    quantity: f64,
    values: *const f64,
    len: usize,
) -> *mut PortfolioAsset {
    if name.is_null() {
        return ptr::null_mut();
    }

    let slice = slice::from_raw_parts(name, name_len as usize);
    let name = String::from_utf8_unchecked(slice.to_vec());

    let input_array = unsafe {
        assert!(!values.is_null());
        slice::from_raw_parts(values, len)
    };

    Box::into_raw(Box::new(PortfolioAsset::new(
        // portfolio_weight,
        name,
        quantity,
        input_array.to_vec(),
    )))
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_destroy(asset: *mut PortfolioAsset) {
    if !asset.is_null() {
        drop(Box::from_raw(asset));
    }
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_apply_rates_of_change(asset: *mut PortfolioAsset) {
    (&mut *asset).apply_rates_of_change();
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_get_mean_and_std(asset: *mut PortfolioAsset) -> Tuple {
    Tuple::from_result((&mut *asset).mean_and_std_dev())
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_current_value(asset: *mut PortfolioAsset) -> f64 {
    (&mut *asset).current_value()
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_current_total_value(asset: *mut PortfolioAsset) -> f64 {
    (&mut *asset).current_total_value()
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_profit_loss(asset: *mut PortfolioAsset) -> NullableFloat {
    NullableFloat::from((&mut *asset).profit_loss())
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_profit(
    asset: *mut PortfolioAsset,
    underlying: NullableFloat,
) -> f64 {
    match underlying.is_valid {
        true => (&mut *asset).profit(Some(underlying.val)),
        false => (&mut *asset).profit(None),
    }
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_payoff(
    asset: *mut PortfolioAsset,
    underlying: NullableFloat,
) -> f64 {
    match underlying.is_valid {
        true => (&mut *asset).payoff(Some(underlying.val)),
        false => (&mut *asset).payoff(None),
    }
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_new() -> *mut Portfolio {
    Box::into_raw(Box::new(Portfolio::from(vec![])))
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_destroy(portfolio: *mut Portfolio) {
    if !portfolio.is_null() {
        drop(Box::from_raw(portfolio));
    }
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_add_asset(
    portfolio: *mut Portfolio,
    asset: *mut PortfolioAsset,
) {
    (&mut *portfolio).add_asset((*asset).clone());
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_apply_rates_of_change(portfolio: *mut Portfolio) {
    (&mut *portfolio).apply_rates_of_change()
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_valid_sizes(portfolio: *mut Portfolio) -> bool {
    (&mut *portfolio).valid_sizes()
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_size(portfolio: *mut Portfolio) -> usize {
    (&mut *portfolio).size()
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_profit_loss(asset: *mut Portfolio) -> NullableFloat {
    NullableFloat::from((&mut *asset).profit_loss())
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_payoff(asset: *mut Portfolio, underlying: NullableFloat) -> f64 {
    match underlying.is_valid {
        true => (&mut *asset).payoff(Some(underlying.val)),
        false => (&mut *asset).payoff(None),
    }
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_is_valid(portfolio: *mut Portfolio) -> bool {
    (&mut *portfolio).is_valid()
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_get_mean_and_std(portfolio: *mut Portfolio) -> Tuple {
    match (&mut *portfolio).mean_and_std_dev() {
        Ok(MuSigma { mean, std_dev }) => Tuple::valid(mean, std_dev),
        Err(_) => Tuple::invalid(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_value_at_risk(
    portfolio: *mut Portfolio,
    confidence: f64,
    initial_investment: f64,
) -> NullableFloat {
    NullableFloat::from((&mut *portfolio).value_at_risk(confidence, initial_investment))
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_value_at_risk_percent(
    portfolio: *mut Portfolio,
    confidence: f64,
) -> NullableFloat {
    NullableFloat::from_result((&mut *portfolio).value_at_risk_pct(confidence))
}
