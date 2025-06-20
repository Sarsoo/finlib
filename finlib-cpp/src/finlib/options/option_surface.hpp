#pragma once

#include <finlib/finlib-native.h>


namespace finlib {
    class OptionsSurface {
    public:
        ~OptionsSurface();

        void generate();

        void par_generate();

    private:
        explicit OptionsSurface(finlibrs::OptionsSurface *handle);

        finlibrs::OptionsSurface *handle;
    };
}
