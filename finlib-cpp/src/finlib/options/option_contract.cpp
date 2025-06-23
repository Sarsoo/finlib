//
// Created by Andy Pack on 19/06/2025.
//

#include "option_contract.hpp"

namespace finlib {
    OptionContract::OptionContract(finlibrs::OptionType option_type,
                                   finlibrs::OptionStyle option_style,
                                   finlibrs::Side side,
                                   double strike,
                                   double premium) {
        handle = finlibrs::option_contract_from(option_type, option_style, side, strike, premium);
    }

    double OptionContract::payoff(double underlying) {
        return finlibrs::option_contract_payoff(handle, underlying);
    }

    double OptionContract::profit(double underlying) {
        return finlibrs::option_contract_profit(handle, underlying);
    }

    bool OptionContract::will_be_exercised(double underlying) {
        return finlibrs::option_contract_will_be_exercised(handle, underlying);
    }

    OptionContract::~OptionContract() {
        finlibrs::option_contract_destroy(handle);
    }
}
