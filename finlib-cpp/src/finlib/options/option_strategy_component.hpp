#pragma once

#include <finlib/finlib-native.h>


namespace finlib {
    class OptionStrategyComponent {
    public:
        explicit OptionStrategyComponent(finlibrs::OptionType option_type,
                                         finlibrs::Side side,
                                         double strike,
                                         double premium);

        double payoff(double underlying);

        double profit(double underlying);

        bool will_be_exercised(double underlying);

        ~OptionStrategyComponent();

    private:
        finlibrs::OptionStrategyComponent *handle;
        friend class OptionStrategy;
    };
}
