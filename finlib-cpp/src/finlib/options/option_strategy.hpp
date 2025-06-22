#pragma once

#include <finlib/finlib-native.h>

#include "option_contract.hpp"


namespace finlib {
    class OptionStrategy {
    public:
        explicit OptionStrategy();

        void add_component(std::unique_ptr<OptionContract> component);

        double payoff(double underlying);

        double profit(double underlying);

        size_t size();

        ~OptionStrategy();

    private:
        finlibrs::OptionStrategy *handle;
    };
}
