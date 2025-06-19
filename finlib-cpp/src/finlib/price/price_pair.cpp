//
// Created by Andy Pack on 19/06/2025.
//

#include "price_pair.hpp"

#include "finlib/finlib-native.h"

namespace finlib {
    PricePair::PricePair(double bid, double offer) {
        handle = finlibrs::price_pair_new(bid, offer);
    }
    PricePair::PricePair(finlibrs::PricePair *handle) {
        this->handle = handle;
    }
    double PricePair::spread() {
        return finlibrs::price_pair_spread(handle);
    }
    double PricePair::midpoint() {
        return finlibrs::price_pair_midpoint(handle);
    }

    PricePair::~PricePair() {
        finlibrs::price_pair_destroy(handle);
    }
} // finlib