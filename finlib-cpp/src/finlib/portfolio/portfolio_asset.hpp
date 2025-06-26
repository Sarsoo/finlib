//
// Created by Andy Pack on 19/06/2025.
//

#pragma once

#include <vector>

#include <finlib/finlib-native.h>

namespace finlib {
    class PortfolioAsset {
    public:
        PortfolioAsset(std::string name, double quantity, finlibrs::TimeSpan timespan);

        explicit PortfolioAsset(finlibrs::PortfolioAsset *handle);

        double current_value();

        finlibrs::NullableFloat current_total_value();

        finlibrs::NullableFloat profit_loss();

        finlibrs::Tuple get_mean_and_std();

        ~PortfolioAsset();

    private:
        finlibrs::PortfolioAsset *handle;

        friend class Portfolio;
    };
}
