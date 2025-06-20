//
// Created by Andy Pack on 19/06/2025.
//

#include "option_surface.hpp"

namespace finlib {
    OptionsSurface::OptionsSurface(finlibrs::OptionsSurface *handle) {
        this->handle = handle;
    }

    OptionsSurface::~OptionsSurface() {
        finlibrs::option_surface_destroy(handle);
    }

    void OptionsSurface::generate() {
        finlibrs::option_surface_generate(handle);
    }

    void OptionsSurface::par_generate() {
        finlibrs::option_surface_par_generate(handle);
    }
}
