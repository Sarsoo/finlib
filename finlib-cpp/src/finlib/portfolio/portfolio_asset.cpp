//
// Created by Andy Pack on 19/06/2025.
//

#include "portfolio_asset.hpp"

#include <vector>

namespace finlib {
    PortfolioAsset::PortfolioAsset(double portfolio_weight, std::string name, std::vector<double> values) {
        handle = finlibrs::portfolio_asset_new(portfolio_weight, reinterpret_cast<uint8_t *>(name.data()),
                                               static_cast<int32_t>(name.length()),
                                               values.data(),
                                               values.size());
    }

    PortfolioAsset::PortfolioAsset(finlibrs::PortfolioAsset *handle) {
        this->handle = handle;
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
