//
// Created by Andy Pack on 19/06/2025.
//

#include "strategy.hpp"

namespace finlib {
    Strategy::Strategy() {
        handle = finlibrs::strategy_new();
    }

    void Strategy::add_component(std::unique_ptr<OptionContract> component) {
        finlibrs::strategy_add_option_component(handle, component->handle);
    }

    void Strategy::add_component(std::unique_ptr<Swap> component) {
        finlibrs::strategy_add_swap_component(handle, component->handle);
    }

    double Strategy::payoff(double underlying) {
        return finlibrs::strategy_payoff(handle, underlying);
    }

    double Strategy::profit(double underlying) {
        return finlibrs::strategy_profit(handle, underlying);
    }

    size_t Strategy::size() {
        return finlibrs::strategy_size(handle);
    }

    Strategy::~Strategy() {
        finlibrs::strategy_destroy(handle);
    }
}
