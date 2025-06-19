use finlib::price::enums::Side;
use finlib::swaps::Swap;
use std::slice;

#[no_mangle]
pub unsafe extern "C" fn swap_from(fixed_rate: f64, fixed_side: Side, premium: f64) -> *mut Swap {
    Box::into_raw(Box::new(Swap::from(fixed_rate, fixed_side, premium)))
}

#[no_mangle]
pub unsafe extern "C" fn swap_net_return(swap: *mut Swap, floating_rate: f64) -> f64 {
    (&mut *swap).net_return(floating_rate)
}

#[no_mangle]
pub unsafe extern "C" fn swap_net_return_from_multiple(
    swap: *mut Swap,
    values: *const f64,
    len: usize,
) -> f64 {
    let input_array: Vec<f64> = unsafe {
        assert!(!values.is_null());
        slice::from_raw_parts(values, len)
    }
    .iter()
    .map(|x| *x)
    .collect();

    (&mut *swap).net_return_from_multiple(input_array)
}

#[no_mangle]
pub unsafe extern "C" fn swap_destroy(swap: *mut Swap) {
    if !swap.is_null() {
        drop(Box::from_raw(swap));
    }
}
