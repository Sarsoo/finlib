pub mod generate;
pub mod option_surface;

pub use generate::*;

use super::{Option, OptionGreeks};

#[cfg(feature = "py")]
use pyo3::prelude::*;
use statrs::distribution::{Continuous, ContinuousCDF, Normal};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct OptionVariables {
    underlying_price: f64,
    strike_price: f64,
    volatility: f64,
    risk_free_interest_rate: f64,
    dividend: f64,
    time_to_expiration: f64,
    d1: std::option::Option<f64>,
    d2: std::option::Option<f64>,
}

impl OptionVariables {
    pub fn from(
        underlying_price: f64,
        strike_price: f64,
        volatility: f64,
        risk_free_interest_rate: f64,
        dividend: f64,
        time_to_expiration: f64,
    ) -> Self {
        Self {
            underlying_price,
            strike_price,
            volatility,
            risk_free_interest_rate,
            dividend,
            time_to_expiration,
            d1: None,
            d2: None,
        }
    }

    pub fn call(mut self) -> CallOption {
        let n = Normal::new(0., 1.0).unwrap();
        let (d1, d2) = self.d1_d2();
        self.d1 = Some(d1);
        self.d2 = Some(d2);

        let first =
            self.underlying_price * (-self.dividend * self.time_to_expiration).exp() * n.cdf(d1);

        let second = self.strike_price
            * (-self.risk_free_interest_rate * self.time_to_expiration).exp()
            * n.cdf(d2);

        CallOption::from(first - second, self)
    }

    pub fn put(mut self) -> PutOption {
        let n = Normal::new(0., 1.0).unwrap();
        let (d1, d2) = self.d1_d2();
        self.d1 = Some(d1);
        self.d2 = Some(d2);

        let first = self.strike_price
            * (-self.risk_free_interest_rate * self.time_to_expiration).exp()
            * n.cdf(-d2);

        let second =
            self.underlying_price * (-self.dividend * self.time_to_expiration).exp() * n.cdf(-d1);

        PutOption::from(first - second, self)
    }

    pub fn d1_d2(&self) -> (f64, f64) {
        let d1 = self.d1();

        (d1, self.d2(d1))
    }

    pub fn d1(&self) -> f64 {
        let first = (self.underlying_price / self.strike_price).log(std::f64::consts::E);

        let second = self.time_to_expiration
            * (self.risk_free_interest_rate - self.dividend + (f64::powi(self.volatility, 2) / 2.));

        let denominator = self.volatility * f64::sqrt(self.time_to_expiration);

        (first + second) / denominator
    }

    pub fn d2(&self, d1: f64) -> f64 {
        d1 - (self.volatility * f64::sqrt(self.time_to_expiration))
    }
}

// #[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct CallOption {
    pub price: f64,
    pub variables: OptionVariables,
    pub greeks: std::option::Option<OptionGreeks>,
}

impl CallOption {
    pub fn from(price: f64, variables: OptionVariables) -> Self {
        Self {
            price,
            variables,
            greeks: None,
        }
    }
}

impl Option for CallOption {
    fn price(&self) -> f64 {
        self.price
    }

    fn strike(&self) -> f64 {
        self.variables.strike_price
    }

    fn delta(&self) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();

        (-self.variables.dividend * self.variables.time_to_expiration).exp()
            * n.cdf(self.variables.d1.unwrap())
    }

    fn gamma(&self) -> f64 {
        gamma(&self.variables)
    }

    fn vega(&self) -> f64 {
        vega(&self.variables)
    }

    fn theta(&self) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();
        let first = theta_first(&self.variables, &n);

        let second = self.variables.risk_free_interest_rate
            * self.variables.strike_price
            * (-self.variables.risk_free_interest_rate * self.variables.time_to_expiration).exp()
            * n.cdf(self.variables.d2.unwrap());

        let third = self.variables.dividend
            * self.variables.underlying_price
            * (-self.variables.dividend * self.variables.time_to_expiration).exp()
            * n.cdf(self.variables.d1.unwrap());

        first - second + third
    }

    fn rho(&self) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();

        self.variables.strike_price
            * self.variables.time_to_expiration
            * (-self.variables.risk_free_interest_rate * self.variables.time_to_expiration).exp()
            * n.cdf(self.variables.d2.unwrap())
    }

    fn calc_greeks(&mut self) {
        self.greeks = Some(OptionGreeks::from(self));
    }

    fn has_greeks(&self) -> bool {
        self.greeks.is_some()
    }
}

// #[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct PutOption {
    pub price: f64,
    pub variables: OptionVariables,
    pub greeks: std::option::Option<OptionGreeks>,
}

impl PutOption {
    pub fn from(price: f64, variables: OptionVariables) -> Self {
        Self {
            price,
            variables,
            greeks: None,
        }
    }
}

impl Option for PutOption {
    fn price(&self) -> f64 {
        self.price
    }

    fn strike(&self) -> f64 {
        self.variables.strike_price
    }

    fn delta(&self) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();

        (-self.variables.dividend * self.variables.time_to_expiration).exp()
            * (n.cdf(self.variables.d1.unwrap()) - 1.)
    }

    fn gamma(&self) -> f64 {
        gamma(&self.variables)
    }

    fn vega(&self) -> f64 {
        vega(&self.variables)
    }

    fn theta(&self) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();
        let first = theta_first(&self.variables, &n);

        let second = self.variables.risk_free_interest_rate
            * self.variables.strike_price
            * (-self.variables.risk_free_interest_rate * self.variables.time_to_expiration).exp()
            * n.cdf(-self.variables.d2.unwrap());

        let third = self.variables.dividend
            * self.variables.underlying_price
            * (-self.variables.dividend * self.variables.time_to_expiration).exp()
            * n.cdf(-self.variables.d1.unwrap());

        first + second - third
    }

    fn rho(&self) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();

        -self.variables.strike_price
            * self.variables.time_to_expiration
            * (-self.variables.risk_free_interest_rate * self.variables.time_to_expiration).exp()
            * n.cdf(-self.variables.d2.unwrap())
    }

    fn calc_greeks(&mut self) {
        self.greeks = Some(OptionGreeks::from(self));
    }

    fn has_greeks(&self) -> bool {
        self.greeks.is_some()
    }
}

fn theta_first(v: &OptionVariables, n: &Normal) -> f64 {
    let numerator = v.underlying_price * v.volatility * (-v.dividend * v.time_to_expiration).exp();
    let denominator = 2. * f64::sqrt(v.time_to_expiration);

    -(numerator / denominator) * n.pdf(v.d1.unwrap())
}

pub fn gamma(v: &OptionVariables) -> f64 {
    let n = Normal::new(0., 1.0).unwrap();

    let numerator = (-v.dividend * v.time_to_expiration).exp();
    let denominator = v.underlying_price * v.volatility * f64::sqrt(v.time_to_expiration);

    (numerator / denominator) * n.pdf(v.d1.unwrap())
}

pub fn vega(v: &OptionVariables) -> f64 {
    let n = Normal::new(0., 1.0).unwrap();

    let numerator = (-v.dividend * v.time_to_expiration).exp();

    v.underlying_price * numerator * f64::sqrt(v.time_to_expiration) * n.pdf(v.d1.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://goodcalculators.com/black-scholes-calculator/

    fn get_example_option() -> OptionVariables {
        OptionVariables::from(100., 100., 0.25, 0.05, 0.01, 30. / 365.25)
    }

    #[test]
    fn call_test() {
        let v = get_example_option();

        let diff = (v.call().price - 3.019).abs();

        assert!(diff < 0.01);
    }

    #[test]
    fn put_test() {
        let v = get_example_option();

        let diff = (v.put().price - 2.691).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn call_delta_test() {
        let v = get_example_option();

        let diff = (v.call().delta() - 0.532).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn put_delta_test() {
        let v = get_example_option();

        let delta = v.put().delta();
        let diff = (delta - -0.467).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn gamma_test() {
        let v = get_example_option();

        let gamma = v.put().gamma();
        let diff = (gamma - 0.055).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn vega_test() {
        let v = get_example_option();

        let vega = v.put().vega();
        let diff = (vega - 11.390).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn call_rho_test() {
        let v = get_example_option();

        let diff = (v.call().rho() - 4.126).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn put_rho_test() {
        let v = get_example_option();

        let rho = v.put().rho();
        let diff = (rho - -4.060).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn call_theta_test() {
        let v = get_example_option();

        let diff = (v.call().theta() - -19.300).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn put_theta_test() {
        let v = get_example_option();

        let theta = v.put().theta();
        let diff = (theta - -15.319).abs();
        assert!(diff < 0.01);
    }
}
