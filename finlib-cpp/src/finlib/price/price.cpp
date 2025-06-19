//
// Created by Andy Pack on 19/06/2025.
//

#include "price.hpp"

#include "finlib/finlib-native.h"

namespace finlib {
    Price::Price(double bid, finlibrs::Side side) {
        handle = finlibrs::price_new(bid, side);
    }

    Price::Price(finlibrs::Price *handle) {
        this->handle = handle;
    }

    Price::~Price() {
        finlibrs::price_destroy(handle);
    }
} // finlib
