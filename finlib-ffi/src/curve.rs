use alloc::boxed::Box;
use chrono::NaiveDate;
use core::ptr;
use finlib::curve::curve::{Curve, CurveType};
use finlib::price::PricePair;

#[no_mangle]
pub unsafe extern "C" fn curve_new(curve_type: CurveType) -> *mut Curve {
    Box::into_raw(Box::new(Curve::new(curve_type)))
}

#[no_mangle]
pub unsafe extern "C" fn curve_size(curve: *mut Curve) -> usize {
    (&mut *curve).size()
}

#[no_mangle]
pub unsafe extern "C" fn curve_add_rate_from(
    curve: *mut Curve,
    bid: f64,
    offer: f64,
    year: i32,
    month: u32,
    day: u32,
) {
    (&mut *curve).add_rate_from(
        bid,
        offer,
        NaiveDate::from_ymd_opt(year, month, day).unwrap(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn curve_get_rate(
    curve: *mut Curve,
    year: i32,
    month: u32,
    day: u32,
) -> *mut PricePair {
    match (&mut *curve).get_rate(NaiveDate::from_ymd_opt(year, month, day).unwrap()) {
        None => ptr::null_mut(),
        Some(v) => Box::into_raw(Box::new(v)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn curve_get_absolute_rate(
    curve: *mut Curve,
    year: i32,
    month: u32,
    day: u32,
) -> *mut PricePair {
    match (&mut *curve).get_absolute_rate(NaiveDate::from_ymd_opt(year, month, day).unwrap()) {
        None => ptr::null_mut(),
        Some(v) => Box::into_raw(Box::new(v)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn curve_get_cumulative_rate(
    curve: *mut Curve,
    year: i32,
    month: u32,
    day: u32,
) -> *mut PricePair {
    match (&mut *curve).get_cumulative_rate(NaiveDate::from_ymd_opt(year, month, day).unwrap()) {
        None => ptr::null_mut(),
        Some(v) => Box::into_raw(Box::new(v)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn curve_get_carry_rate(
    curve: *mut Curve,
    from_year: i32,
    from_month: u32,
    from_day: u32,
    to_year: i32,
    to_month: u32,
    to_day: u32,
) -> *mut PricePair {
    match (&mut *curve).get_carry_rate(
        NaiveDate::from_ymd_opt(from_year, from_month, from_day).unwrap(),
        NaiveDate::from_ymd_opt(to_year, to_month, to_day).unwrap(),
    ) {
        None => ptr::null_mut(),
        Some(v) => Box::into_raw(Box::new(v)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn curve_destroy(curve: *mut Curve) {
    if !curve.is_null() {
        drop(Box::from_raw(curve));
    }
}
