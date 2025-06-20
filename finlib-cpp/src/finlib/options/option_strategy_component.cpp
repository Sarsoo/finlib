//
// Created by Andy Pack on 19/06/2025.
//

#include "option_strategy_component.hpp"

namespace finlib {
    OptionStrategyComponent::OptionStrategyComponent(finlibrs::OptionType option_type,
                                                     finlibrs::Side side,
                                                     double strike,
                                                     double premium) {
        handle = finlibrs::option_strategy_component_from(option_type, side, strike, premium);
    }

    double OptionStrategyComponent::payoff(double underlying) {
        return finlibrs::option_strategy_component_payoff(handle, underlying);
    }

    double OptionStrategyComponent::profit(double underlying) {
        return finlibrs::option_strategy_component_profit(handle, underlying);
    }

    bool OptionStrategyComponent::will_be_exercised(double underlying) {
        return finlibrs::option_strategy_component_will_be_exercised(handle, underlying);
    }

    OptionStrategyComponent::~OptionStrategyComponent() {
        finlibrs::option_strategy_component_destroy(handle);
    }
}
