using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace FinLib.Risk;

public static class ValueAtRisk
{
    public static double Historical(IEnumerable<double> values, double confidence)
    {
        unsafe {
            var valueArr = values.ToArray();
            fixed (double* ptrOne = valueArr) {
                var ret = NativeMethods.value_at_risk(ptrOne, (UIntPtr)valueArr.Length, confidence);

                return *ret;
            }
        }
    }
}