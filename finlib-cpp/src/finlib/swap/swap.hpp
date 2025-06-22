//
// Created by Andy Pack on 19/06/2025.
//

#pragma once
#include "finlib/finlib-native.h"
#include <vector>

namespace finlib {
    class Swap {
    public:
        Swap(double fixed_rate, finlibrs::Side side, double premium);

        double fixed_rate(double floating_rate) const;

        double fixed_rate_from_multiple(std::vector<double> values) const;

        ~Swap();

    private:
        finlibrs::Swap *handle;
        friend class Strategy;
    };
} // finlib
