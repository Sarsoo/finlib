pub mod curve;
pub mod indicators;
pub mod options;
pub mod portfolio;
pub mod price;
pub mod swap;

use std::slice;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    one: f64,
    two: f64,
    is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NullableFloat {
    val: f64,
    is_valid: bool,
}

#[no_mangle]
pub extern "C" fn interest_compound(principal: f64, rate: f64, time: f64, n: f64) -> f64 {
    finlib::interest::compound(principal, rate, time, n)
}

#[no_mangle]
pub unsafe extern "C" fn covariance(
    arr: *const f64,
    len: usize,
    arr_two: *const f64,
    len_two: usize,
) -> NullableFloat {
    let input_array = unsafe {
        assert!(!arr.is_null());
        slice::from_raw_parts(arr, len)
    };

    let input_array_two = unsafe {
        assert!(!arr_two.is_null());
        slice::from_raw_parts(arr_two, len_two)
    };

    match finlib::stats::covariance(input_array, input_array_two) {
        None => NullableFloat {
            val: 0.0,
            is_valid: false,
        },
        Some(v) => NullableFloat {
            val: v,
            is_valid: true,
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn historical_value_at_risk(
    arr: *const f64,
    len: usize,
    confidence: f64,
) -> NullableFloat {
    let input_array = unsafe {
        assert!(!arr.is_null());
        slice::from_raw_parts(arr, len)
    };

    NullableFloat {
        val: finlib::risk::var::historical::value_at_risk(input_array, confidence),
        is_valid: true,
    }
}

#[no_mangle]
pub unsafe extern "C" fn varcovar_value_at_risk(
    arr: *const f64,
    len: usize,
    confidence: f64,
) -> NullableFloat {
    let input_array = unsafe {
        assert!(!arr.is_null());
        slice::from_raw_parts(arr, len)
    };

    NullableFloat {
        val: finlib::risk::var::varcovar::value_at_risk_percent(input_array, confidence),
        is_valid: true,
    }
}

#[no_mangle]
pub unsafe extern "C" fn scale_value_at_risk(initial_value: f64, time_cycles: isize) -> f64 {
    finlib::risk::var::varcovar::scale_value_at_risk(initial_value, time_cycles)
}
