use std::ptr::null;
use std::slice;

#[no_mangle]
pub extern "C" fn interest_compound(principal: f64, rate: f64, time: f64, n: f64) -> f64 {
    finlib::interest::compound(principal, rate, time, n)
}

#[no_mangle]
pub unsafe extern "C" fn covariance(arr: *const f64, len: usize, arr_two: *const f64, len_two: usize) -> *const f64 {
    let input_array = unsafe {
        assert!(!arr.is_null());
        slice::from_raw_parts(arr, len)
    };

    let input_array_two = unsafe {
        assert!(!arr_two.is_null());
        slice::from_raw_parts(arr_two, len_two)
    };

    match finlib::stats::covariance(input_array, input_array_two) {
        None => null(),
        Some(v) => Box::into_raw(Box::new(v))
    }

}

#[no_mangle]
pub unsafe extern "C" fn historical_value_at_risk(arr: *const f64, len: usize, confidence: f64) -> *const f64 {
    let input_array = unsafe {
        assert!(!arr.is_null());
        slice::from_raw_parts(arr, len)
    };

    Box::into_raw(Box::new(finlib::risk::var::historical::value_at_risk(input_array, confidence)))
}

#[no_mangle]
pub unsafe extern "C" fn varcovar_value_at_risk(arr: *const f64, len: usize, confidence: f64) -> *const f64 {
    let input_array = unsafe {
        assert!(!arr.is_null());
        slice::from_raw_parts(arr, len)
    };

    Box::into_raw(Box::new(finlib::risk::var::varcovar::value_at_risk(input_array, confidence)))
}
