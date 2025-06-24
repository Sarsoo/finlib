use crate::fixed_income::annuity::{
    future_value_ordinary, monthly_payment, present_value_ordinary, total_interest_repayment,
    total_repayment,
};
use crate::price::payoff::{Payoff, Profit};
use bon::Builder;
#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "py", pyclass(get_all, eq, ord))]
#[cfg_attr(feature = "ffi", repr(C))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Builder, Clone, Debug, PartialEq, PartialOrd)]
pub struct Mortgage {
    pub purchase_price: f64,
    pub deposit: f64,
    pub interest_rate: f64,
    pub term_years: i32,
}

impl Mortgage {
    pub fn ltv(&self) -> f64 {
        (self.purchase_price - self.deposit) / self.purchase_price
    }

    pub fn loan_value(&self) -> f64 {
        self.purchase_price - self.deposit
    }

    pub fn monthly_payment(&self) -> f64 {
        monthly_payment(self.loan_value(), self.interest_rate, self.term_years)
    }

    pub fn total_repayment(&self) -> f64 {
        total_repayment(self.loan_value(), self.interest_rate, self.term_years)
    }

    pub fn total_interest_repayment(&self) -> f64 {
        total_interest_repayment(self.loan_value(), self.interest_rate, self.term_years)
    }

    /// Total value of the mortgage to the lender
    pub fn future_value(&self, annual_interest_rate: f64) -> f64 {
        future_value_ordinary(
            self.monthly_payment(),
            annual_interest_rate,
            self.term_years,
            12,
        )
    }

    /// Total interest to the lender accrued via its future value
    pub fn net_future_value_interest(&self, annual_interest_rate: f64) -> f64 {
        let fv = future_value_ordinary(
            self.monthly_payment(),
            annual_interest_rate,
            self.term_years,
            12,
        );

        fv - self.total_repayment()
    }

    /// Total interest to the lender including that repaid by the debtor and that accrued via its future value
    pub fn total_interest(&self, annual_interest_rate: f64) -> f64 {
        let fv = future_value_ordinary(
            self.monthly_payment(),
            annual_interest_rate,
            self.term_years,
            12,
        );

        fv - self.loan_value()
    }

    pub fn present_value(&self) -> f64 {
        present_value_ordinary(
            self.monthly_payment(),
            self.interest_rate,
            self.term_years,
            12,
        )
    }
}

impl Payoff<f64> for Mortgage {
    fn payoff(&self, _: f64) -> f64 {
        self.total_repayment()
    }
}

impl Profit<f64> for Mortgage {
    fn profit(&self, _: f64) -> f64 {
        self.total_interest_repayment()
    }
}
