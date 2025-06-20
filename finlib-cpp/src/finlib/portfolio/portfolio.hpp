//
// Created by Andy Pack on 19/06/2025.
//

#pragma once

#include <finlib/finlib-native.h>

#include "portfolio_asset.hpp"

namespace finlib {
    class Portfolio {
    public:
        Portfolio();

        void add_asset(std::unique_ptr<PortfolioAsset> asset);

        void apply_rates_of_change();

        bool valid_sizes();

        finlibrs::NullableFloat profit_loss();

        size_t size();

        // bool valid_weights();

        bool is_valid();

        finlibrs::Tuple get_mean_and_std();

        finlibrs::NullableFloat value_at_risk(double confidence, double initial_investment);

        finlibrs::NullableFloat value_at_risk_percent(double confidence);

        ~Portfolio();

    private:
        finlibrs::Portfolio *handle;
    };
}
