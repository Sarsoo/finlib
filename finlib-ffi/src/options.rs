use finlib::options::blackscholes::OptionVariables;

#[no_mangle]
pub unsafe extern "C" fn option_vars_from(
    underlying_price: f64,
    strike_price: f64,
    volatility: f64,
    risk_free_interest_rate: f64,
    dividend: f64,
    time_to_expiration: f64,
) -> *mut OptionVariables {
    Box::into_raw(Box::new(OptionVariables::from(
        underlying_price,
        strike_price,
        volatility,
        risk_free_interest_rate,
        dividend,
        time_to_expiration,
    )))
}

#[no_mangle]
pub unsafe extern "C" fn option_vars_destroy(option: *mut OptionVariables) {
    if !option.is_null() {
        drop(Box::from_raw(option));
    }
}
