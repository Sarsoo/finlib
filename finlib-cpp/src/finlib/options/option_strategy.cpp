//
// Created by Andy Pack on 19/06/2025.
//

#include "option_strategy.hpp"

namespace finlib {
    OptionStrategy::OptionStrategy() {
        handle = finlibrs::option_strategy_new();
    }

    void OptionStrategy::add_component(std::unique_ptr<OptionContract> component) {
        finlibrs::option_strategy_add_component(handle, component->handle);
    }

    double OptionStrategy::payoff(double underlying) {
        return finlibrs::option_strategy_payoff(handle, underlying);
    }

    double OptionStrategy::profit(double underlying) {
        return finlibrs::option_strategy_profit(handle, underlying);
    }

    size_t OptionStrategy::size() {
        return finlibrs::option_strategy_size(handle);
    }

    OptionStrategy::~OptionStrategy() {
        finlibrs::option_strategy_destroy(handle);
    }
}
