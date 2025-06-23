pub mod curve;
pub mod indicators;
pub mod mortgage;
pub mod options;
pub mod portfolio;
pub mod price;
pub mod strategy;
pub mod swap;

use finlib::stats::MuSigma;
use std::slice;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    one: f64,
    two: f64,
    is_valid: bool,
}

impl Tuple {
    pub fn from(val: Option<(f64, f64)>) -> Self {
        match val {
            None => Self::invalid(),
            Some((one, two)) => Tuple::valid(one, two),
        }
    }

    pub fn from_result(val: Result<MuSigma, ()>) -> Self {
        match val {
            Err(_) => Self::invalid(),
            Ok(MuSigma {
                mean: one,
                std_dev: two,
            }) => Tuple::valid(one, two),
        }
    }

    pub fn valid(one: f64, two: f64) -> Self {
        Self {
            one,
            two,
            is_valid: true,
        }
    }

    pub fn invalid() -> Self {
        Self {
            one: 0.0,
            two: 0.0,
            is_valid: false,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NullableFloat {
    pub val: f64,
    pub is_valid: bool,
}

impl NullableFloat {
    pub fn from(val: Option<f64>) -> Self {
        match val {
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

    pub fn from_result(val: Result<f64, ()>) -> Self {
        match val {
            Err(_) => NullableFloat {
                val: 0.0,
                is_valid: false,
            },
            Ok(v) => NullableFloat {
                val: v,
                is_valid: true,
            },
        }
    }

    pub fn to_option(&self) -> Option<f64> {
        match self.is_valid {
            true => Some(self.val),
            false => None,
        }
    }
}

#[no_mangle]
pub extern "C" fn interest_compound(principal: f64, rate: f64, time: f64, n: f64) -> f64 {
    finlib::interest::compounded_principal(principal, rate, time, n)
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

    NullableFloat::from(finlib::stats::covariance(input_array, input_array_two))
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
        val: finlib::risk::var::historical::value_at_risk_percent(input_array, confidence),
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
        val: finlib::risk::var::varcovar::value_at_risk_percent_1d(input_array, confidence),
        is_valid: true,
    }
}

#[no_mangle]
pub unsafe extern "C" fn scale_value_at_risk(initial_value: f64, time_cycles: isize) -> f64 {
    finlib::risk::var::scale_value_at_risk(initial_value, time_cycles)
}
