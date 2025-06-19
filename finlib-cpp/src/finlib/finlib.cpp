//
// Created by Andy Pack on 21/02/2025.
//

#include "finlib.hpp"

#include <iostream>

namespace finlib {
    void init() {
        std::cout << "Initializing..." << std::endl;

        auto ret = finlibrs::interest_compound(100, .05, 1, 1.0);

        std::cout << "Answer: " << ret << std::endl;
    }
}
