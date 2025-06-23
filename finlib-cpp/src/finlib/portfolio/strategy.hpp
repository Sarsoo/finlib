#pragma once

#include <memory>

#include <finlib/finlib-native.h>

#include "../options/option_contract.hpp"
#include "swap/swap.hpp"


namespace finlib {
    class Strategy {
    public:
        explicit Strategy();

        void add_component(std::unique_ptr<OptionContract> component);

        void add_component(std::unique_ptr<Swap> component);

        double payoff(double underlying);

        double profit(double underlying);

        size_t size();

        ~Strategy();

    private:
        finlibrs::Strategy *handle;
    };
}
