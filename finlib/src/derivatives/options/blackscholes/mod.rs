pub mod generate;
pub mod option_surface;

use bon::Builder;
pub use generate::*;

use super::{OptionContract, OptionGreeks, OptionType};

#[cfg(feature = "py")]
use pyo3::prelude::*;
use statrs::distribution::{Continuous, ContinuousCDF, Normal};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use core::f64;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Builder, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct OptionVariables {
    pub option_type: OptionType,
    pub underlying_price: f64,
    pub strike_price: f64,
    pub volatility: f64,
    pub risk_free_interest_rate: f64,
    pub dividend: f64,
    pub time_to_expiration: f64,
}

pub struct BlackscholesPricer {}

impl BlackscholesPricer {
    pub fn price(&self, variables: &OptionVariables) -> f64 {
        match variables.option_type {
            OptionType::Call => self.price_call(variables),
            OptionType::Put => self.price_put(variables),
        }
    }

    pub fn set_price(&self, option: &mut OptionContract, variables: &OptionVariables) {
        option.premium = self.price(variables);
    }

    pub fn set_greeks(&self, option: &mut OptionContract, variables: &OptionVariables) {
        option.greeks = Some(
            OptionGreeks::builder()
                .delta(self.delta(&variables))
                .rho(self.delta(&variables))
                .vega(self.delta(&variables))
                .theta(self.delta(&variables))
                .gamma(self.delta(&variables))
                .build(),
        );
    }

    pub fn price_put(&self, variables: &OptionVariables) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();
        let (d1, d2) = self.d1_d2(variables);

        let first = variables.strike_price
            * (-variables.risk_free_interest_rate * variables.time_to_expiration).exp()
            * n.cdf(-d2);

        let second = variables.underlying_price
            * (-variables.dividend * variables.time_to_expiration).exp()
            * n.cdf(-d1);

        first - second
    }

    pub fn price_call(&self, variables: &OptionVariables) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();
        let (d1, d2) = self.d1_d2(variables);

        let first = variables.underlying_price
            * (-variables.dividend * variables.time_to_expiration).exp()
            * n.cdf(d1);

        let second = variables.strike_price
            * (-variables.risk_free_interest_rate * variables.time_to_expiration).exp()
            * n.cdf(d2);

        first - second
    }

    pub fn d1_d2(&self, variables: &OptionVariables) -> (f64, f64) {
        let d1 = self.d1(variables);

        (d1, self.d2(variables, d1))
    }

    pub fn d1(&self, variables: &OptionVariables) -> f64 {
        let first = (variables.underlying_price / variables.strike_price).log(f64::consts::E);

        let second = variables.time_to_expiration
            * (variables.risk_free_interest_rate - variables.dividend
                + (f64::powi(variables.volatility, 2) / 2.));

        let denominator = variables.volatility * f64::sqrt(variables.time_to_expiration);

        (first + second) / denominator
    }

    pub fn d2(&self, variables: &OptionVariables, d1: f64) -> f64 {
        d1 - (variables.volatility * f64::sqrt(variables.time_to_expiration))
    }

    pub fn delta(&self, variables: &OptionVariables) -> f64 {
        match variables.option_type {
            OptionType::Call => self.delta_call(variables),
            OptionType::Put => self.delta_put(variables),
        }
    }

    fn delta_call(&self, v: &OptionVariables) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();

        (-v.dividend * v.time_to_expiration).exp() * n.cdf(self.d1(v))
    }

    fn delta_put(&self, v: &OptionVariables) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();

        (-v.dividend * v.time_to_expiration).exp() * (n.cdf(self.d1(v)) - 1.)
    }

    pub fn theta(&self, variables: &OptionVariables) -> f64 {
        match variables.option_type {
            OptionType::Call => self.theta_call(variables),
            OptionType::Put => self.theta_put(variables),
        }
    }

    fn theta_call(&self, v: &OptionVariables) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();
        let first = self.theta_first(&v, &n);

        let d1 = self.d1(v);

        let second = v.risk_free_interest_rate
            * v.strike_price
            * (-v.risk_free_interest_rate * v.time_to_expiration).exp()
            * n.cdf(self.d2(v, d1));

        let third = v.dividend
            * v.underlying_price
            * (-v.dividend * v.time_to_expiration).exp()
            * n.cdf(d1);

        first - second + third
    }

    fn theta_put(&self, v: &OptionVariables) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();
        let first = self.theta_first(&v, &n);

        let d1 = self.d1(v);

        let second = v.risk_free_interest_rate
            * v.strike_price
            * (-v.risk_free_interest_rate * v.time_to_expiration).exp()
            * n.cdf(-self.d2(v, d1));

        let third = v.dividend
            * v.underlying_price
            * (-v.dividend * v.time_to_expiration).exp()
            * n.cdf(-d1);

        first + second - third
    }

    pub fn rho(&self, variables: &OptionVariables) -> f64 {
        match variables.option_type {
            OptionType::Call => self.rho_call(variables),
            OptionType::Put => self.rho_put(variables),
        }
    }

    fn rho_call(&self, v: &OptionVariables) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();

        v.strike_price
            * v.time_to_expiration
            * (-v.risk_free_interest_rate * v.time_to_expiration).exp()
            * n.cdf(self.d2(v, self.d1(v)))
    }

    fn rho_put(&self, v: &OptionVariables) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();

        -v.strike_price
            * v.time_to_expiration
            * (-v.risk_free_interest_rate * v.time_to_expiration).exp()
            * n.cdf(-self.d2(v, self.d1(v)))
    }

    fn theta_first(&self, v: &OptionVariables, n: &Normal) -> f64 {
        let numerator =
            v.underlying_price * v.volatility * (-v.dividend * v.time_to_expiration).exp();
        let denominator = 2. * f64::sqrt(v.time_to_expiration);

        -(numerator / denominator) * n.pdf(self.d1(v))
    }

    pub fn gamma(&self, v: &OptionVariables) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();

        let numerator = (-v.dividend * v.time_to_expiration).exp();
        let denominator = v.underlying_price * v.volatility * f64::sqrt(v.time_to_expiration);

        (numerator / denominator) * n.pdf(self.d1(v))
    }

    pub fn vega(&self, v: &OptionVariables) -> f64 {
        let n = Normal::new(0., 1.0).unwrap();

        let numerator = (-v.dividend * v.time_to_expiration).exp();

        v.underlying_price * numerator * f64::sqrt(v.time_to_expiration) * n.pdf(self.d1(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://goodcalculators.com/black-scholes-calculator/

    fn get_example_option(option_type: OptionType) -> OptionVariables {
        // OptionVariables::from(100., 100., 0.25, 0.05, 0.01, 30. / 365.25)
        OptionVariables::builder()
            .underlying_price(100.)
            .strike_price(100.)
            .volatility(0.25)
            .risk_free_interest_rate(0.05)
            .dividend(0.01)
            .time_to_expiration(30. / 365.25)
            .option_type(option_type)
            .build()
    }

    #[test]
    fn call_test() {
        let v = get_example_option(OptionType::Call);

        let pricer = BlackscholesPricer {};
        let diff = (pricer.price(&v) - 3.019).abs();

        assert!(diff < 0.01);
    }

    #[test]
    fn put_test() {
        let v = get_example_option(OptionType::Put);

        let pricer = BlackscholesPricer {};
        let diff = (pricer.price(&v) - 2.691).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn call_delta_test() {
        let v = get_example_option(OptionType::Call);

        let pricer = BlackscholesPricer {};
        let diff = (pricer.delta(&v) - 0.532).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn put_delta_test() {
        let v = get_example_option(OptionType::Put);

        let pricer = BlackscholesPricer {};
        let diff = (pricer.delta(&v) - -0.467).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn gamma_test() {
        let v = get_example_option(OptionType::Put);

        let pricer = BlackscholesPricer {};
        let diff = (pricer.gamma(&v) - 0.055).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn vega_test() {
        let v = get_example_option(OptionType::Call);

        let pricer = BlackscholesPricer {};
        let diff = (pricer.vega(&v) - 11.390).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn call_rho_test() {
        let v = get_example_option(OptionType::Call);

        let pricer = BlackscholesPricer {};
        let diff = (pricer.rho(&v) - 4.126).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn put_rho_test() {
        let v = get_example_option(OptionType::Put);

        let pricer = BlackscholesPricer {};
        let diff = (pricer.rho(&v) - -4.060).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn call_theta_test() {
        let v = get_example_option(OptionType::Call);

        let pricer = BlackscholesPricer {};
        let diff = (pricer.theta(&v) - -19.300).abs();
        assert!(diff < 0.01);
    }

    #[test]
    fn put_theta_test() {
        let v = get_example_option(OptionType::Put);

        let pricer = BlackscholesPricer {};
        let diff = (pricer.theta(&v) - -15.319).abs();
        assert!(diff < 0.01);
    }
}
