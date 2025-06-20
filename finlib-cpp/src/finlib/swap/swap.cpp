//
// Created by Andy Pack on 19/06/2025.
//

#include "swap.hpp"

namespace finlib {
    Swap::Swap(double fixed_rate, finlibrs::Side side, double premium) {
        handle = finlibrs::swap_from(fixed_rate, side, premium);
    }

    double Swap::fixed_rate(double floating_rate) const {
        return finlibrs::swap_payoff(handle, floating_rate);
    }

    double Swap::fixed_rate_from_multiple(std::vector<double> values) const {
        return finlibrs::swap_payoff_from_multiple(handle, values.data(), values.size());
    }

    Swap::~Swap() {
        finlibrs::swap_destroy(handle);
    }
} // finlib
