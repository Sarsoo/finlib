use crate::fixed_income::annuity::{monthly_payment, total_interest_payment, total_repayment};
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
    pub fn monthly_payment(&self) -> f64 {
        monthly_payment(
            self.purchase_price - self.deposit,
            self.interest_rate,
            self.term_years,
        )
    }

    pub fn total_repayment(&self) -> f64 {
        total_repayment(
            self.purchase_price - self.deposit,
            self.interest_rate,
            self.term_years,
        )
    }

    pub fn total_interest_payment(&self) -> f64 {
        total_interest_payment(
            self.purchase_price - self.deposit,
            self.interest_rate,
            self.term_years,
        )
    }
}
