use chrono::NaiveDate;
use finlib::curve::curve::Curve;
use finlib::price::price::PricePair;

#[no_mangle]
pub unsafe extern "C" fn curve_new() -> *mut Curve {
    Box::into_raw(Box::new(Curve::new()))
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
pub unsafe extern "C" fn curve_get_cumulative_rate(
    curve: *mut Curve,
    year: i32,
    month: u32,
    day: u32,
) -> *mut PricePair {
    Box::into_raw(Box::new((&mut *curve).get_cumulative_rate(
        NaiveDate::from_ymd_opt(year, month, day).unwrap(),
    )))
}

#[no_mangle]
pub unsafe extern "C" fn curve_destroy(curve: *mut Curve) {
    if !curve.is_null() {
        drop(Box::from_raw(curve));
    }
}
