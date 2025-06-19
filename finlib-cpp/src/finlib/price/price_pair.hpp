//
// Created by Andy Pack on 19/06/2025.
//

#pragma once

#include <finlib/finlib-native.h>

namespace finlib {

class PricePair {
public:
    PricePair(double bid, double offer);
    explicit PricePair(finlibrs::PricePair* handle);
    ~PricePair();
    double spread();
    double midpoint();
private:
    finlibrs::PricePair* handle;
};

} // finlib


