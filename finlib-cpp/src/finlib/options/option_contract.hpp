#pragma once

#include <finlib/finlib-native.h>


namespace finlib {
    class OptionContract {
    public:
        explicit OptionContract(finlibrs::OptionType option_type,
                                finlibrs::Side side,
                                double strike,
                                double premium);

        double payoff(double underlying);

        double profit(double underlying);

        bool will_be_exercised(double underlying);

        ~OptionContract();

    private:
        finlibrs::OptionContract *handle;
        friend class Strategy;
    };
}
