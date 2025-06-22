use console_log;
use log::Level;
use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen(start)]
// fn start() {
//
// }

#[wasm_bindgen]
pub fn init_logging() {
    if let Err(_) = console_log::init_with_level(Level::Debug) {}
}

#[wasm_bindgen]
pub struct RelativeStrengthIndicator {}

#[wasm_bindgen]
impl RelativeStrengthIndicator {
    pub fn relative_strength_indicator(
        time_period: f64,
        average_gain: f64,
        average_loss: f64,
    ) -> f64 {
        finlib::indicators::rsi::relative_strength_indicator(
            time_period,
            average_gain,
            average_loss,
        )
    }

    pub fn relative_strength_indicator_smoothed(
        time_period: f64,
        previous_average_gain: f64,
        current_gain: f64,
        previous_average_loss: f64,
        current_loss: f64,
    ) -> f64 {
        finlib::indicators::rsi::relative_strength_indicator_smoothed(
            time_period,
            previous_average_gain,
            current_gain,
            previous_average_loss,
            current_loss,
        )
    }
}

#[wasm_bindgen]
pub struct Interest {}

#[wasm_bindgen]
impl Interest {
    pub fn compound(principal: f64, rate: f64, time: f64, n: f64) -> f64 {
        finlib::interest::compound(principal, rate, time, n)
    }
}

#[wasm_bindgen]
pub struct ValueAtRisk {}

#[wasm_bindgen]
impl ValueAtRisk {
    pub fn historical(values: Vec<f64>, confidence: f64) -> f64 {
        finlib::risk::var::historical::value_at_risk_percent(&values, confidence)
    }

    pub fn varcovar(values: Vec<f64>, confidence: f64) -> f64 {
        finlib::risk::var::varcovar::value_at_risk_percent_1d(&values, confidence)
    }

    pub fn varcovar_from_initial_investment_1d(
        values: Vec<f64>,
        confidence: f64,
        initial_investment: f64,
    ) -> f64 {
        finlib::risk::var::varcovar::value_at_risk_from_initial_investment_1d(
            &values,
            confidence,
            initial_investment,
        )
    }

    pub fn scale_value_at_risk(initial_value: f64, time_cycles: isize) -> f64 {
        finlib::risk::var::scale_value_at_risk(initial_value, time_cycles)
    }
}

#[wasm_bindgen]
pub struct Stats {}

#[wasm_bindgen]
impl Stats {
    pub fn mean(slice: Vec<f64>) -> f64 {
        finlib::stats::mean(&slice)
    }

    pub fn population_variance(slice: Vec<f64>) -> f64 {
        finlib::stats::population_variance(&slice)
    }

    pub fn sample_variance(slice: Vec<f64>) -> f64 {
        finlib::stats::sample_variance(&slice)
    }

    pub fn population_std_dev(slice: Vec<f64>) -> f64 {
        finlib::stats::population_std_dev(&slice)
    }

    pub fn sample_std_dev(slice: Vec<f64>) -> f64 {
        finlib::stats::sample_std_dev(&slice)
    }

    pub fn inverse_cdf_value(confidence: f64, mean: f64, std_dev: f64) -> f64 {
        finlib::stats::inverse_cdf_value(confidence, mean, std_dev)
    }

    pub fn covariance(slice: Vec<f64>, slice_two: Vec<f64>) -> Option<f64> {
        finlib::stats::covariance(&slice, &slice_two)
    }
}

#[wasm_bindgen]
pub struct Util {}

#[wasm_bindgen]
impl Util {
    pub fn changes(slice: Vec<f64>) -> Vec<f64> {
        finlib::util::roc::changes(&slice).collect::<Vec<_>>()
    }

    pub fn rates_of_change(slice: Vec<f64>) -> Vec<f64> {
        finlib::util::roc::rates_of_change(&slice).collect::<Vec<_>>()
    }

    pub fn dot_product(a: Vec<f64>, b: Vec<f64>) -> f64 {
        finlib::util::vector::dot_product(&a, &b)
    }

    pub fn mag(slice: Vec<f64>) -> f64 {
        finlib::util::vector::mag(&slice)
    }
}
