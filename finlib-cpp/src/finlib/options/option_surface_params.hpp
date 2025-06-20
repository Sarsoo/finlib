#pragma once

#include <finlib/finlib-native.h>
#include "option_surface.hpp"

namespace finlib {
    class OptionSurfaceParameters {
    public:
        explicit OptionSurfaceParameters(
            std::tuple<int, int> underlying_price_range, std::tuple<double, double> underlying_price_min_max,
            std::tuple<int, int> strike_price_range, std::tuple<double, double> strike_price_min_max,
            std::tuple<int, int> volatility_range, std::tuple<double, double> volatility_min_max,
            std::tuple<int, int> risk_free_interest_rate_range,
            std::tuple<double, double> risk_free_interest_rate_min_max,
            std::tuple<int, int> dividend_range, std::tuple<double, double> dividend_min_max,
            std::tuple<int, int> time_to_expiration_range, std::tuple<double, double> time_to_expiration_min_max
        );

        std::unique_ptr<OptionsSurface> walk();

        ~OptionSurfaceParameters();

    private:
        finlibrs::OptionSurfaceParameters *handle;
    };
}
