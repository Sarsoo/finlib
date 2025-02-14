using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace FinLib.Interest;

public static class Interest
{
    public static double Compound(double principal, double rate, double time, double n)
    {
        return NativeMethods.interest_compound(principal, rate, time, n);
    }
}