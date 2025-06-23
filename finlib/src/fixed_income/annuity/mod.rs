use crate::interest::{compound_rate_per_n, compound_rate_per_n_neg, rate_per_n};
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "py", pyfunction)]
pub fn monthly_payment(principal: f64, annual_interest_rate: f64, term_years: i32) -> f64 {
    payment(principal, annual_interest_rate, term_years, 12)
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn payment(principal: f64, interest_rate: f64, term: i32, payments_per_term: i32) -> f64 {
    let compounded_rate = compound_rate_per_n(interest_rate, term as f64, payments_per_term as f64);

    (principal * compounded_rate / (compounded_rate - 1.))
        * (rate_per_n(interest_rate, payments_per_term as f64) - 1.)
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn future_value(
    cash_flow_per_period: f64,
    interest_rate: f64,
    term: i32,
    payments_per_term: i32,
) -> f64 {
    let monthly_interest_rate = interest_rate / (payments_per_term as f64);

    cash_flow_per_period
        * (compound_rate_per_n(interest_rate, term as f64, payments_per_term as f64) - 1.)
        / monthly_interest_rate
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn present_value(
    cash_flow_per_period: f64,
    interest_rate: f64,
    term: i32,
    payments_per_term: i32,
) -> f64 {
    let monthly_interest_rate = interest_rate / (payments_per_term as f64);

    cash_flow_per_period
        * (1. - compound_rate_per_n_neg(interest_rate, term as f64, payments_per_term as f64))
        / monthly_interest_rate
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn total_repayment(principal: f64, annual_interest_rate: f64, term_years: i32) -> f64 {
    monthly_payment(principal, annual_interest_rate, term_years) * 12. * (term_years as f64)
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn total_interest_payment(principal: f64, annual_interest_rate: f64, term_years: i32) -> f64 {
    total_repayment(principal, annual_interest_rate, term_years) - principal
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn monthly_interest_payment(principal: f64, annual_interest_rate: f64) -> f64 {
    principal * annual_interest_rate / 12.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payment_example() {
        let p = monthly_payment(200000., 0.065, 30);

        assert_eq!((p * 100.).round() / 100., 1264.14);
    }

    #[test]
    fn payment_example_2() {
        let p = monthly_payment(30000., 0.03, 4);

        assert_eq!((p * 100.).round() / 100., 664.03);
    }

    #[test]
    fn payment_example_3() {
        let p = monthly_payment(500000., 0.06, 30);

        assert_eq!((p * 100.).round() / 100., 2997.75);
    }

    #[test]
    fn monthly_interest_payment_test() {
        let p = monthly_interest_payment(30000., 0.03);

        assert_eq!((p * 100.).round() / 100., 75.);
    }

    #[test]
    fn future_value_test() {
        let p = future_value(125000., 0.08, 5, 1);

        assert_eq!(p.round(), 733325.);
    }

    // #[test]
    // fn future_value_equal_total() {
    //     let monthly_payment = monthly_payment(125000., 0.08, 5);
    //
    //     let p = future_value(monthly_payment, 0.08, 5, 12);
    //     let p2 = total_repayment(125000., 0.08, 5);
    //
    //     assert_eq!(p, p2);
    // }

    #[test]
    fn current_value_test() {
        let p = present_value(1000., 0.05, 5, 1);

        assert_eq!(((p * 100.).round()) / 100., 4329.48);
    }
}
