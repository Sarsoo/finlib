use crate::fixed_income::mortgage::Mortgage;
use crate::price::payoff::Payoff;
use pyo3::prelude::*;

#[pymethods]
impl Mortgage {
    #[new]
    pub fn init(purchase_price: f64, deposit: f64, interest_rate: f64, term_years: i32) -> Self {
        Mortgage::builder()
            .term_years(term_years)
            .interest_rate(interest_rate)
            .deposit(deposit)
            .purchase_price(purchase_price)
            .build()
    }

    #[getter(ltv)]
    pub fn ltv_py(&self) -> f64 {
        self.ltv()
    }

    #[getter(loan_value)]
    pub fn loan_value_py(&self) -> f64 {
        self.loan_value()
    }

    #[getter(monthly_payment)]
    pub fn monthly_payment_py(&self) -> f64 {
        self.monthly_payment()
    }

    #[getter(total_repayment)]
    pub fn total_repayment_py(&self) -> f64 {
        self.total_repayment()
    }

    #[getter(total_interest_repayment)]
    pub fn total_interest_repayment_py(&self) -> f64 {
        self.total_interest_repayment()
    }

    #[getter(present_value)]
    pub fn present_value_py(&self) -> f64 {
        self.present_value()
    }

    #[pyo3(name = "future_value")]
    pub fn future_value_py(&mut self, annual_interest_rate: f64) -> f64 {
        self.future_value(annual_interest_rate)
    }

    #[pyo3(name = "net_future_value_interest")]
    pub fn net_future_value_interest_py(&mut self, annual_interest_rate: f64) -> f64 {
        self.net_future_value_interest(annual_interest_rate)
    }

    #[pyo3(name = "total_interest")]
    pub fn total_interest_py(&mut self, annual_interest_rate: f64) -> f64 {
        self.total_interest(annual_interest_rate)
    }
}
