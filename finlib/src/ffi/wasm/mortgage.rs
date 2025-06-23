use crate::fixed_income::mortgage::Mortgage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Mortgage {
    #[wasm_bindgen(constructor)]
    pub fn init_wasm(
        purchase_price: f64,
        deposit: f64,
        interest_rate: f64,
        term_years: i32,
    ) -> Self {
        Mortgage::builder()
            .term_years(term_years)
            .interest_rate(interest_rate)
            .deposit(deposit)
            .purchase_price(purchase_price)
            .build()
    }

    #[wasm_bindgen(getter = ltv)]
    pub fn ltv_wasm(&self) -> f64 {
        self.ltv()
    }

    #[wasm_bindgen(getter = loanValue)]
    pub fn loan_value_wasm(&self) -> f64 {
        self.loan_value()
    }

    #[wasm_bindgen(getter = monthlyPayment)]
    pub fn monthly_payment_wasm(&self) -> f64 {
        self.monthly_payment()
    }

    #[wasm_bindgen(getter = totalRepayment)]
    pub fn total_repayment_wasm(&self) -> f64 {
        self.total_repayment()
    }

    #[wasm_bindgen(getter = totalInterestRepayment)]
    pub fn total_interest_repayment_wasm(&self) -> f64 {
        self.total_interest_repayment()
    }

    #[wasm_bindgen(getter = presentValue)]
    pub fn present_value_wasm(&self) -> f64 {
        self.present_value()
    }

    #[wasm_bindgen(js_name = "futureValue")]
    pub fn future_value_wasm(&self, annual_interest_rate: f64) -> f64 {
        self.future_value(annual_interest_rate)
    }

    #[wasm_bindgen(js_name = "netFutureValueInterest")]
    pub fn net_future_value_interest_wasm(&self, annual_interest_rate: f64) -> f64 {
        self.net_future_value_interest(annual_interest_rate)
    }

    #[wasm_bindgen(js_name = "totalInterest")]
    pub fn total_interest_wasm(&self, annual_interest_rate: f64) -> f64 {
        self.total_interest(annual_interest_rate)
    }
}
