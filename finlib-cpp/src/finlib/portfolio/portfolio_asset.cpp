//
// Created by Andy Pack on 19/06/2025.
//

#include "portfolio_asset.hpp"

#include <vector>

namespace finlib {
    PortfolioAsset::PortfolioAsset(std::string name, double quantity, std::vector<double> values) {
        handle = finlibrs::portfolio_asset_new(reinterpret_cast<uint8_t *>(name.data()),
                                               static_cast<int32_t>(name.length()),
                                               quantity,
                                               values.data(),
                                               values.size());
    }

    PortfolioAsset::PortfolioAsset(finlibrs::PortfolioAsset *handle) {
        this->handle = handle;
    }

    double PortfolioAsset::current_value() {
        return finlibrs::portfolio_asset_current_value(handle);
    }

    double PortfolioAsset::current_total_value() {
        return finlibrs::portfolio_asset_current_total_value(handle);
    }

    finlibrs::NullableFloat PortfolioAsset::profit_loss() {
        return finlibrs::portfolio_asset_profit_loss(handle);
    }

    void PortfolioAsset::apply_rates_of_change() {
        finlibrs::portfolio_asset_apply_rates_of_change(handle);
    }

    finlibrs::Tuple PortfolioAsset::get_mean_and_std() {
        return finlibrs::portfolio_asset_get_mean_and_std(handle);
    }

    PortfolioAsset::~PortfolioAsset() {
        if (handle)
            finlibrs::portfolio_asset_destroy(handle);
    }
}
