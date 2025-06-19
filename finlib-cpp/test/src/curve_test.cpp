#include "curve/curve.hpp"

#include <gtest/gtest.h>

using namespace std;


TEST(CurveTest, BasicCreation) {
    auto curve = new finlib::Curve(finlibrs::CurveType::Absolute);

    delete curve;
}

TEST(CurveTest, Size) {
    auto curve = new finlib::Curve(finlibrs::CurveType::Absolute);

    ASSERT_EQ(curve->size(), 0);

    delete curve;
}

TEST(CurveTest, Add) {
    auto curve = std::make_unique<finlib::Curve>(finlibrs::CurveType::Absolute);

    ASSERT_EQ(curve->size(), 0);

    auto date = std::chrono::year_month_day(2020y, chrono::January, 1d);
    curve->add_rate_from(100, 100, date);

    ASSERT_EQ(curve->size(), 1);

    curve->add_rate_from(100, 100, date);

    ASSERT_EQ(curve->size(), 1);
}
