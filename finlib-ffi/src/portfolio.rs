use std::{ptr, slice};
use finlib::risk::portfolio::{Portfolio, PortfolioAsset};
use crate::{NullableFloat, Tuple};

#[no_mangle]
pub unsafe extern "C" fn portfolio_asset_new(portfolio_weight: f64, name: *const u8, name_len: i32, values: *const f64, len: usize) -> *mut PortfolioAsset {

    if name.is_null() {
        return ptr::null_mut();
    }

    let slice = slice::from_raw_parts(name, name_len as usize);
    let name = String::from_utf8_unchecked(slice.to_vec());

    let input_array = unsafe {
        assert!(!values.is_null());
        slice::from_raw_parts(values, len)
    };


    Box::into_raw(Box::new(PortfolioAsset::new(portfolio_weight, name, input_array.to_vec())))
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
    match (&mut *asset).get_mean_and_std() {
        None => Tuple { one: 0.0, two: 0.0, is_valid: false },
        Some((one, two)) => {
            Tuple {
                one, two, is_valid: true
            }
        }
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
pub unsafe extern "C" fn portfolio_add_asset(portfolio: *mut Portfolio, asset: *mut PortfolioAsset) {
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
pub unsafe extern "C" fn portfolio_valid_weights(portfolio: *mut Portfolio) -> bool {
    (&mut *portfolio).valid_weights()
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_is_valid(portfolio: *mut Portfolio) -> bool {
    (&mut *portfolio).is_valid()
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_get_mean_and_std(portfolio: *mut Portfolio) -> Tuple {
    match (&mut *portfolio).get_mean_and_std() {
        None => Tuple { one: 0.0, two: 0.0, is_valid: false },
        Some((one, two)) => {
            Tuple {
                one, two, is_valid: true
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_value_at_risk(portfolio: *mut Portfolio, confidence: f64, initial_investment: f64) -> NullableFloat {
    match (&mut *portfolio).value_at_risk(confidence, initial_investment) {
        None => NullableFloat { val: 0.0, is_valid: false },
        Some(v) => NullableFloat { val: v, is_valid: true }
    }
}

#[no_mangle]
pub unsafe extern "C" fn portfolio_value_at_risk_percent(portfolio: *mut Portfolio, confidence: f64) -> NullableFloat {
    match (&mut *portfolio).value_at_risk_percent(confidence) {
        None => NullableFloat { val: 0.0, is_valid: false },
        Some(v) => NullableFloat { val: v, is_valid: true }
    }
}
