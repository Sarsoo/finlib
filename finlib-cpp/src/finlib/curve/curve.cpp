//
// Created by Andy Pack on 19/06/2025.
//

#include "curve.hpp"

namespace finlib {
    Curve::Curve(finlibrs::CurveType type) {
        handle = curve_new(type);
    }

    Curve::~Curve() {
        finlibrs::curve_destroy(handle);
    }

    size_t Curve::size() const {
        return finlibrs::curve_size(handle);
    }

    void Curve::add_rate_from(double bid, double offer, std::chrono::year_month_day &date) {
        finlibrs::curve_add_rate_from(handle, bid, offer, static_cast<int32_t>(date.year()),
                                      static_cast<uint32_t>(date.month()), static_cast<uint32_t>(date.day()));
    }

    std::unique_ptr<PricePair> Curve::get_rate(std::chrono::year_month_day &date) {
        auto price_handle = finlibrs::curve_get_rate(handle, static_cast<int32_t>(date.year()),
                                                     static_cast<uint32_t>(date.month()),
                                                     static_cast<uint32_t>(date.day()));
        if (price_handle) {
            return std::make_unique<PricePair>(price_handle);
        } else {
            return nullptr;
        }
    }

    std::unique_ptr<PricePair> Curve::get_absolute_rate(std::chrono::year_month_day &date) {
        auto price_handle = finlibrs::curve_get_absolute_rate(handle, static_cast<int32_t>(date.year()),
                                                              static_cast<uint32_t>(date.month()),
                                                              static_cast<uint32_t>(date.day()));
        if (price_handle) {
            return std::make_unique<PricePair>(price_handle);
        } else {
            return nullptr;
        }
    }

    std::unique_ptr<PricePair> Curve::get_cumulative_rate(std::chrono::year_month_day &date) {
        auto price_handle = finlibrs::curve_get_cumulative_rate(handle, static_cast<int32_t>(date.year()),
                                                                static_cast<uint32_t>(date.month()),
                                                                static_cast<uint32_t>(date.day()));
        if (price_handle) {
            return std::make_unique<PricePair>(price_handle);
        } else {
            return nullptr;
        }
    }

    std::unique_ptr<PricePair>
    Curve::get_carry_rate(std::chrono::year_month_day &from, std::chrono::year_month_day &to) {
        auto price_handle = finlibrs::curve_get_carry_rate(handle, static_cast<int32_t>(from.year()),
                                                           static_cast<uint32_t>(from.month()),
                                                           static_cast<uint32_t>(from.day()),
                                                           static_cast<int32_t>(to.year()),
                                                           static_cast<uint32_t>(to.month()),
                                                           static_cast<uint32_t>(to.day()));
        if (price_handle) {
            return std::make_unique<PricePair>(price_handle);
        } else {
            return nullptr;
        }
    }
}
