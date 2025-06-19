//
// Created by Andy Pack on 19/06/2025.
//

#pragma once

#include <finlib/finlib-native.h>

namespace finlib {
    class PortfolioAsset {
    public:
        PortfolioAsset(double portfolio_weight, std::string name, std::vector<double> values);

        explicit PortfolioAsset(finlibrs::PortfolioAsset *handle);

        void apply_rates_of_change();

        finlibrs::Tuple get_mean_and_std();

        ~PortfolioAsset();

    private:
        finlibrs::PortfolioAsset *handle;

        friend class Portfolio;
    };
}
