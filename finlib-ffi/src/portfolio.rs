use crate::{NullableFloat, Tuple};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use chrono::DateTime;
use core::slice;
use finlib::market_data::price_range::PriceTimestamp;
use finlib::portfolio::{Portfolio, PortfolioAsset};
use finlib::price::payoff::Payoff;
use finlib::price::PricePair;
#[cfg(feature = "std")]
use finlib::risk::var::ValueAtRisk;
use finlib::stats::{MuSigma, PopulationStats};

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
pub unsafe extern "C" fn portfolio_valid_sizes(portfolio: *mut Portfolio) -> bool {
    (&mut *portfolio).valid_sizes()
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_size(portfolio: *mut Portfolio) -> usize {
    (&mut *portfolio).size()
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_profit_loss(asset: *mut Portfolio) -> NullableFloat {
    NullableFloat::from_result((&mut *asset).profit_loss())
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
pub unsafe extern "C" fn portfolio_initial_investment(portfolio: *mut Portfolio) -> NullableFloat {
    NullableFloat::from_result((&mut *portfolio).initial_investment())
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_add_price(
    portfolio: *mut Portfolio,
    key: *const u8,
    key_len: i32,
    price: *mut PriceTimestamp,
) {
    if key.is_null() {
        return;
    }

    let slice = slice::from_raw_parts(key, key_len as usize);
    let key = String::from_utf8_unchecked(slice.to_vec());

    let _ = (&mut *portfolio).add_price(key, (*price).clone());
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_add_price_pair(
    portfolio: *mut Portfolio,
    key: *const u8,
    key_len: i32,
    price: *mut PricePair,
    timestamp: i64,
) {
    if key.is_null() {
        return;
    }

    let slice = slice::from_raw_parts(key, key_len as usize);
    let key = String::from_utf8_unchecked(slice.to_vec());

    if let Some(t) = DateTime::from_timestamp_millis(timestamp) {
        let _ = (&mut *portfolio).add_price_pair(key, (*price).clone(), t);
    }
}

#[cfg(feature = "std")]
#[no_mangle]
pub unsafe extern "C" fn portfolio_get_mean_and_std(portfolio: *mut Portfolio) -> Tuple {
    match (&mut *portfolio).mean_and_std_dev() {
        Ok(MuSigma { mean, std_dev }) => Tuple::valid(mean, std_dev),
        Err(_) => Tuple::invalid(),
    }
}

#[cfg(feature = "std")]
#[no_mangle]
pub unsafe extern "C" fn portfolio_value_at_risk(
    portfolio: *mut Portfolio,
    confidence: f64,
    initial_investment: NullableFloat,
) -> NullableFloat {
    NullableFloat::from_result(
        (&mut *portfolio).value_at_risk(confidence, initial_investment.to_option()),
    )
}

#[cfg(feature = "std")]
#[no_mangle]
pub unsafe extern "C" fn portfolio_value_at_risk_percent(
    portfolio: *mut Portfolio,
    confidence: f64,
) -> NullableFloat {
    NullableFloat::from_result((&mut *portfolio).value_at_risk_pct(confidence))
}

#[cfg(feature = "std")]
#[no_mangle]
pub unsafe extern "C" fn portfolio_value_at_risk_afer_time(
    portfolio: *mut Portfolio,
    confidence: f64,
    initial_investment: NullableFloat,
    at: isize,
) -> NullableFloat {
    NullableFloat::from_result((&mut *portfolio).value_at_risk_after_time(
        confidence,
        initial_investment.to_option(),
        at,
    ))
}
