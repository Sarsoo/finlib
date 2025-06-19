//
// Created by Andy Pack on 19/06/2025.
//

#pragma once

#include <finlib/finlib-native.h>

namespace finlib {
    class Price {
    public:
        Price(double bid, finlibrs::Side side);

        explicit Price(finlibrs::Price *handle);

        ~Price();

    private:
        finlibrs::Price *handle;
    };
} // finlib


