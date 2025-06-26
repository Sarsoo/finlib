use crate::{NullableFloat, Tuple};
use alloc::boxed::Box;
use alloc::string::String;
use chrono::DateTime;
use core::ptr;
use core::slice;
use finlib::market_data::price_range::{PriceRangePair, PriceTimestamp};
use finlib::market_data::TimeSpan;
use finlib::portfolio::PortfolioAsset;
use finlib::price::payoff::{Payoff, Profit};
use finlib::price::PricePair;
#[cfg(feature = "std")]
use finlib::risk::var::ValueAtRisk;
use finlib::stats::PopulationStats;

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_new(
    name: *const u8,
    name_len: i32,
    quantity: f64,
    time_scale: TimeSpan,
) -> *mut PortfolioAsset {
    if name.is_null() {
        return ptr::null_mut();
    }

    let slice = slice::from_raw_parts(name, name_len as usize);
    let name = String::from_utf8_unchecked(slice.to_vec());

    Box::into_raw(Box::new(PortfolioAsset::new(name, quantity, time_scale)))
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_destroy(asset: *mut PortfolioAsset) {
    if !asset.is_null() {
        drop(Box::from_raw(asset));
    }
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_get_mean_and_std(asset: *mut PortfolioAsset) -> Tuple {
    Tuple::from_result((&mut *asset).mean_and_std_dev())
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_current_value(
    asset: *mut PortfolioAsset,
) -> *mut PriceRangePair {
    match (&mut *asset).current_value() {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_current_total_value(
    asset: *mut PortfolioAsset,
) -> NullableFloat {
    NullableFloat::from_result((&mut *asset).current_total_value())
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_profit_loss(asset: *mut PortfolioAsset) -> NullableFloat {
    NullableFloat::from_result((&mut *asset).profit_loss())
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

#[cfg(feature = "std")]
#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_value_at_risk(
    portfolio: *mut PortfolioAsset,
    confidence: f64,
    initial_investment: NullableFloat,
) -> NullableFloat {
    NullableFloat::from_result(
        (&mut *portfolio).value_at_risk(confidence, initial_investment.to_option()),
    )
}

#[cfg(feature = "std")]
#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_value_at_risk_percent(
    portfolio: *mut PortfolioAsset,
    confidence: f64,
) -> NullableFloat {
    NullableFloat::from_result((&mut *portfolio).value_at_risk_pct(confidence))
}

#[cfg(feature = "std")]
#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_value_at_risk_afer_time(
    portfolio: *mut PortfolioAsset,
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

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_add_price(
    portfolio: *mut PortfolioAsset,
    price: *mut PriceTimestamp,
) {
    let _ = (&mut *portfolio).add_price((*price).clone());
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_add_price_pair(
    portfolio: *mut PortfolioAsset,
    price: *mut PricePair,
    timestamp: i64,
) {
    if let Some(t) = DateTime::from_timestamp_millis(timestamp) {
        let _ = (&mut *portfolio).add_price_pair((*price).clone(), t);
    }
}
