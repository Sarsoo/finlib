
#[no_mangle]
pub extern "C" fn relative_strength_indicator(time_period: f64, average_gain: f64, average_loss: f64)  -> f64 {
    finlib::indicators::rsi::relative_strength_indicator(time_period, average_gain, average_loss)
}

#[no_mangle]
pub extern "C" fn relative_strength_indicator_smoothed(time_period: f64, previous_average_gain: f64, current_gain: f64, previous_average_loss: f64, current_loss: f64)  -> f64 {
    finlib::indicators::rsi::relative_strength_indicator_smoothed(time_period, previous_average_gain, current_gain, previous_average_loss, current_loss)
}