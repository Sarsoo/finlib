//
// Created by Andy Pack on 19/06/2025.
//

#include "portfolio.hpp"

namespace finlib {
    Portfolio::Portfolio() {
        handle = finlibrs::portfolio_new();
    }

    void Portfolio::add_asset(std::unique_ptr<PortfolioAsset> asset) {
        finlibrs::portfolio_add_asset(handle, asset->handle);
    }

    void Portfolio::apply_rates_of_change() {
        finlibrs::portfolio_apply_rates_of_change(handle);
    }

    bool Portfolio::valid_sizes() {
        return finlibrs::portfolio_valid_sizes(handle);
    }

    finlibrs::NullableFloat Portfolio::profit_loss() {
        return finlibrs::portfolio_profit_loss(handle);
    }

    size_t Portfolio::size() {
        return finlibrs::portfolio_size(handle);
    }

    // bool Portfolio::valid_weights() {
    //     return finlibrs::portfolio_valid_weights(handle);
    // }

    bool Portfolio::is_valid() {
        return finlibrs::portfolio_is_valid(handle);
    }

    finlibrs::Tuple Portfolio::get_mean_and_std() {
        return finlibrs::portfolio_get_mean_and_std(handle);
    }

    finlibrs::NullableFloat Portfolio::value_at_risk(double confidence, finlibrs::NullableFloat initial_investment) {
        return finlibrs::portfolio_value_at_risk(handle, confidence, initial_investment);
    }

    finlibrs::NullableFloat Portfolio::value_at_risk_afer_time(double confidence,
                                                               finlibrs::NullableFloat initial_investment,
                                                               ptrdiff_t after_time) {
        return finlibrs::portfolio_value_at_risk_afer_time(handle, confidence, initial_investment, after_time);
    }


    finlibrs::NullableFloat Portfolio::value_at_risk_percent(double confidence) {
        return finlibrs::portfolio_value_at_risk_percent(handle, confidence);
    }

    Portfolio::~Portfolio() {
        finlibrs::portfolio_destroy(handle);
    }
}
