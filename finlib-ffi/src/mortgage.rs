use finlib::fixed_income::mortgage::Mortgage;

#[no_mangle]
pub unsafe extern "C" fn mortgage_new(
    purchase_price: f64,
    deposit: f64,
    interest_rate: f64,
    term_years: i32,
) -> *mut Mortgage {
    Box::into_raw(Box::new(
        Mortgage::builder()
            .term_years(term_years)
            .interest_rate(interest_rate)
            .deposit(deposit)
            .purchase_price(purchase_price)
            .build(),
    ))
}

#[no_mangle]
pub unsafe extern "C" fn mortgage_ltv(mortgage: *mut Mortgage) -> f64 {
    (*mortgage).ltv()
}

#[no_mangle]
pub unsafe extern "C" fn mortgage_loan_value(mortgage: *mut Mortgage) -> f64 {
    (*mortgage).loan_value()
}

#[no_mangle]
pub unsafe extern "C" fn mortgage_monthly_payment(mortgage: *mut Mortgage) -> f64 {
    (*mortgage).monthly_payment()
}

#[no_mangle]
pub unsafe extern "C" fn mortgage_total_repayment(mortgage: *mut Mortgage) -> f64 {
    (*mortgage).total_repayment()
}

#[no_mangle]
pub unsafe extern "C" fn mortgage_total_interest_repayment(mortgage: *mut Mortgage) -> f64 {
    (*mortgage).total_interest_repayment()
}

#[no_mangle]
pub unsafe extern "C" fn mortgage_present_value(mortgage: *mut Mortgage) -> f64 {
    (*mortgage).present_value()
}

#[no_mangle]
pub unsafe extern "C" fn mortgage_future_value(
    mortgage: *mut Mortgage,
    annual_interest_rate: f64,
) -> f64 {
    (*mortgage).future_value(annual_interest_rate)
}

#[no_mangle]
pub unsafe extern "C" fn mortgage_net_future_value_interest(
    mortgage: *mut Mortgage,
    annual_interest_rate: f64,
) -> f64 {
    (*mortgage).net_future_value_interest(annual_interest_rate)
}

#[no_mangle]
pub unsafe extern "C" fn mortgage_total_interest(
    mortgage: *mut Mortgage,
    annual_interest_rate: f64,
) -> f64 {
    (*mortgage).total_interest(annual_interest_rate)
}

#[no_mangle]
pub unsafe extern "C" fn mortgage_destroy(mortgage: *mut Mortgage) {
    if !mortgage.is_null() {
        drop(Box::from_raw(mortgage));
    }
}
