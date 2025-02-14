#pragma once

/* Generated with cbindgen:0.28.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


namespace finlib {


extern "C" {

const double *covariance(const double *arr, size_t len, const double *arr_two, size_t len_two);

const double *historical_value_at_risk(const double *arr, size_t len, double confidence);

double interest_compound(double principal, double rate, double time, double n);

const double *varcovar_value_at_risk(const double *arr, size_t len, double confidence);

}  // extern "C"

}  // namespace finlib
