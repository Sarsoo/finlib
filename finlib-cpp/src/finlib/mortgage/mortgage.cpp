//
// Created by Andy Pack on 19/06/2025.
//

#include "mortgage.hpp"

namespace finlib {
    Mortgage::Mortgage(double purchase_price,
                       double deposit,
                       double interest_rate,
                       int32_t term_years) {
        handle = finlibrs::mortgage_new(purchase_price, deposit, interest_rate, term_years);
    }

    double Mortgage::ltv() const {
        return finlibrs::mortgage_ltv(handle);
    }

    double Mortgage::loan_value() const {
        return finlibrs::mortgage_loan_value(handle);
    }

    double Mortgage::monthly_payment() const {
        return finlibrs::mortgage_monthly_payment(handle);
    }

    double Mortgage::total_repayment() const {
        return finlibrs::mortgage_total_repayment(handle);
    }

    double Mortgage::total_interest_repayment() const {
        return finlibrs::mortgage_total_interest_repayment(handle);
    }

    double Mortgage::present_value() const {
        return finlibrs::mortgage_present_value(handle);
    }

    double Mortgage::future_value(double annual_interest_rate) const {
        return finlibrs::mortgage_future_value(handle, annual_interest_rate);
    }

    double Mortgage::net_future_value_interest(double annual_interest_rate) const {
        return finlibrs::mortgage_net_future_value_interest(handle, annual_interest_rate);
    }

    double Mortgage::total_interest(double annual_interest_rate) const {
        return finlibrs::mortgage_total_interest(handle, annual_interest_rate);
    }

    Mortgage::~Mortgage() {
        finlibrs::mortgage_destroy(handle);
    }
} // finlib
