#pragma once

#include <finlib/finlib-native.h>

#include "price/price_pair.hpp"
#include <chrono>

namespace finlib {
    class Curve {
    public:
        explicit Curve(finlibrs::CurveType type);

        ~Curve();

        [[nodiscard]] size_t size() const;

        void add_rate_from(double bid, double offer, std::chrono::year_month_day &date);

        std::unique_ptr<PricePair> get_rate(std::chrono::year_month_day &date);

        std::unique_ptr<PricePair> get_absolute_rate(std::chrono::year_month_day &date);

        std::unique_ptr<PricePair> get_cumulative_rate(std::chrono::year_month_day &date);

        std::unique_ptr<PricePair> get_carry_rate(std::chrono::year_month_day &from, std::chrono::year_month_day &to);

    private:
        finlibrs::Curve *handle;
    };
}
