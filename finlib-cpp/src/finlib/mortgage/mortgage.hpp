//
// Created by Andy Pack on 19/06/2025.
//

#pragma once
#include "finlib/finlib-native.h"
#include <vector>

namespace finlib {
    class Mortgage {
    public:
        Mortgage(double purchase_price,
                 double deposit,
                 double interest_rate,
                 int32_t term_years);

        double ltv() const;

        double loan_value() const;

        double monthly_payment() const;

        double total_repayment() const;

        double total_interest_repayment() const;

        double present_value() const;

        double future_value(double annual_interest_rate) const;

        double net_future_value_interest(double annual_interest_rate) const;

        double total_interest(double annual_interest_rate) const;

        ~Mortgage();

    private:
        finlibrs::Mortgage *handle;
    };
} // finlib
