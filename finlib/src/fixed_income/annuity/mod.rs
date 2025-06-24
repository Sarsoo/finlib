use crate::interest::{anticompound_rate_per_n, compound_rate_per_n, rate_per_n};
#[cfg(feature = "py")]
use pyo3::prelude::*;

/// Calculate the constant monthly payment for an annuity on `principal` (100,000) with an `interest_rate` (0.05 for 5%) over a `term` (10 years)
#[cfg_attr(feature = "py", pyfunction)]
pub fn monthly_payment(principal: f64, annual_interest_rate: f64, term_years: i32) -> f64 {
    payment(principal, annual_interest_rate, term_years, 12)
}

/// Calculate the constant periodical payment for an annuity on `principal` (100,000) with an `interest_rate` (0.05 for 5%) over a `term` (10 years) with `payments_per_term` (12 for monthly payments)
#[cfg_attr(feature = "py", pyfunction)]
pub fn payment(principal: f64, interest_rate: f64, term: i32, payments_per_term: i32) -> f64 {
    let compounded_rate = compound_rate_per_n(interest_rate, term as f64, payments_per_term as f64);

    (principal * compounded_rate / (compounded_rate - 1.))
        * (rate_per_n(interest_rate, payments_per_term as f64) - 1.)
}

/// Calculate the future value on an ordinary annuity with a periodical payment `cash_flow_per_period` when you can earn `interest_rate` on the returned payments over a `term` with `payments_per_term`
///
/// The future value identifies the total value of an annuity payment at the end of its term including the possible interest yield on periodical payments as they are delivered
///
/// The future value will be higher than the total repayment by the debtor as it includes further interest yield by the lender on returned payments
///
/// An ordinary annuity entails payments with interest being made at the end of a period
#[cfg_attr(feature = "py", pyfunction)]
pub fn future_value_ordinary(
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

/// Calculate the future value on a due annuity with a periodical payment `cash_flow_per_period` when you can earn `interest_rate` on the returned payments over a `term` with `payments_per_term`
///
/// The future value identifies the total value of an annuity payment at the end of its term including the possible interest yield on periodical payments as they are delivered
///
/// The future value will be higher than the total repayment by the debtor as it includes further interest yield by the lender on returned payments
///
/// A due annuity entails payments with interest being made at the start of a period
#[cfg_attr(feature = "py", pyfunction)]
pub fn future_value_due(
    cash_flow_per_period: f64,
    interest_rate: f64,
    term: i32,
    payments_per_term: i32,
) -> f64 {
    future_value_ordinary(cash_flow_per_period, interest_rate, term, payments_per_term)
        * (1. + (interest_rate / (payments_per_term as f64)))
}

/// Calculate the present value on an ordinary annuity with a periodical payment `cash_flow_per_period` when you can earn `interest_rate` on the returned payments over a `term` with `payments_per_term`
///
/// The present value identifies the total value of an annuity payment at the beginning of its term assuming you can earn interest on present capital to cover later payments
///
/// The present value will be less than the total repayment as the debtor is able to earn interest yield on the held capital before it is used to pay the annuity
///
/// An ordinary annuity entails payments with interest being made at the end of a period
#[cfg_attr(feature = "py", pyfunction)]
pub fn present_value_ordinary(
    cash_flow_per_period: f64,
    interest_rate: f64,
    term: i32,
    payments_per_term: i32,
) -> f64 {
    let monthly_interest_rate = interest_rate / (payments_per_term as f64);

    cash_flow_per_period
        * (1. - anticompound_rate_per_n(interest_rate, term as f64, payments_per_term as f64))
        / monthly_interest_rate
}

/// Calculate the present value on an ordinary annuity with a periodical payment `cash_flow_per_period` when you can earn `interest_rate` on the returned payments over a `term` with `payments_per_term`
///
/// The present value identifies the total value of an annuity payment at the beginning of its term assuming you can earn interest on present capital to cover later payments
///
/// The present value will be less than the total repayment as the debtor is able to earn interest yield on the held capital before it is used to pay the annuity
///
/// A due annuity entails payments with interest being made at the start of a period
#[cfg_attr(feature = "py", pyfunction)]
pub fn present_value_due(
    cash_flow_per_period: f64,
    interest_rate: f64,
    term: i32,
    payments_per_term: i32,
) -> f64 {
    present_value_ordinary(cash_flow_per_period, interest_rate, term, payments_per_term)
        * (1. + (interest_rate / (payments_per_term as f64)))
}

/// Total repayment made on an annuity including interest on principal.
///
/// Equal to the monthly payment multiplied by its term length
///
/// This will be less than the future value as it excludes the possible interest yield for the lender on returned payments
///
/// This will be more than the current value as it excludes the possible interest yield for the debtor on capital for future payments
#[cfg_attr(feature = "py", pyfunction)]
pub fn total_repayment(principal: f64, annual_interest_rate: f64, term_years: i32) -> f64 {
    monthly_payment(principal, annual_interest_rate, term_years) * 12. * (term_years as f64)
}

#[cfg_attr(feature = "py", pyfunction)]
pub fn total_interest_repayment(principal: f64, annual_interest_rate: f64, term_years: i32) -> f64 {
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
    fn future_value_ordinary_test() {
        let p = future_value_ordinary(125000., 0.08, 5, 1);

        assert_eq!(p.round(), 733325.);
    }

    #[test]
    fn future_value_due_test() {
        let p = future_value_due(1000., 0.05, 5, 1);

        assert_eq!(((p * 100.).round()) / 100., 5801.91);
    }

    #[test]
    fn present_value_ordinary_test() {
        let p = present_value_ordinary(1000., 0.05, 5, 1);

        assert_eq!(((p * 100.).round()) / 100., 4329.48);
    }

    #[test]
    fn present_value_due_test() {
        let p = present_value_due(1000., 0.05, 5, 1);

        assert_eq!(((p * 100.).round()) / 100., 4545.95);
    }

    // #[test]
    // fn present_value_no_interest() {
    //     let p = monthly_payment(125000., 0.05, 25);
    //
    //     let total = total_repayment(125000., 0.05, 25);
    //
    //     let future = future_value_ordinary(p, 0., 25, 12);
    //     let present = present_value_ordinary(p, 0., 25, 12);
    //
    //     assert_eq!(present, total);
    //     assert_eq!(total, future);
    // }

    #[test]
    fn present_future_total_payment_comparison() {
        let p = monthly_payment(125000., 0.05, 25);

        let total = total_repayment(125000., 0.05, 25);

        let future = future_value_ordinary(p, 0.05, 25, 12);
        let present = present_value_ordinary(p, 0.05, 25, 12);

        assert!(present < total);
        assert!(total < future);
    }
}
