//
// Created by Andy Pack on 19/06/2025.
//

#include "option_surface_params.hpp"

namespace finlib {
    OptionSurfaceParameters::OptionSurfaceParameters(std::tuple<int, int> underlying_price_range,
                                                     std::tuple<double, double> underlying_price_min_max,
                                                     std::tuple<int, int> strike_price_range,
                                                     std::tuple<double, double> strike_price_min_max,
                                                     std::tuple<int, int> volatility_range,
                                                     std::tuple<double, double> volatility_min_max,
                                                     std::tuple<int, int> risk_free_interest_rate_range,
                                                     std::tuple<double, double> risk_free_interest_rate_min_max,
                                                     std::tuple<int, int> dividend_range,
                                                     std::tuple<double, double> dividend_min_max,
                                                     std::tuple<int, int> time_to_expiration_range,
                                                     std::tuple<double, double> time_to_expiration_min_max) {
        handle = finlibrs::option_surface_parameters_from(
            static_cast<ptrdiff_t>(std::get<0>(underlying_price_range)),
            static_cast<ptrdiff_t>(std::get<1>(underlying_price_range)),
            std::get<0>(underlying_price_min_max),
            std::get<1>(underlying_price_min_max),
            static_cast<ptrdiff_t>(std::get<0>(strike_price_range)),
            static_cast<ptrdiff_t>(std::get<1>(strike_price_range)),
            std::get<0>(strike_price_min_max),
            std::get<1>(strike_price_min_max),
            static_cast<ptrdiff_t>(std::get<0>(volatility_range)),
            static_cast<ptrdiff_t>(std::get<1>(volatility_range)),
            std::get<0>(volatility_min_max),
            std::get<1>(volatility_min_max),
            static_cast<ptrdiff_t>(std::get<0>(risk_free_interest_rate_range)),
            static_cast<ptrdiff_t>(std::get<1>(risk_free_interest_rate_range)),
            std::get<0>(risk_free_interest_rate_min_max),
            std::get<1>(risk_free_interest_rate_min_max),
            static_cast<ptrdiff_t>(std::get<0>(dividend_range)),
            static_cast<ptrdiff_t>(std::get<1>(dividend_range)),
            std::get<0>(dividend_min_max),
            std::get<1>(dividend_min_max),
            static_cast<ptrdiff_t>(std::get<0>(time_to_expiration_range)),
            static_cast<ptrdiff_t>(std::get<1>(time_to_expiration_range)),
            std::get<0>(time_to_expiration_min_max),
            std::get<1>(time_to_expiration_min_max)
        );
    }

    OptionSurfaceParameters::~OptionSurfaceParameters() {
        finlibrs::option_surface_parameters_destroy(handle);
    }

    std::unique_ptr<OptionsSurface> OptionSurfaceParameters::walk() {
        auto surface_handle = finlibrs::option_surface_parameters_walk(handle);

        return std::make_unique<OptionsSurface>(surface_handle);
    }
}
