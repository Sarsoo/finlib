using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace FinLib;

public static class FinLib
{
    public static double CompoundInterest(double principal, double rate, double time, double n)
    {
        return NativeMethods.interest_compound(principal, rate, time, n);
    }

    public static double? Covariance(IEnumerable<double> valuesOne, IEnumerable<double> valuesTwo)
    {
        unsafe {
            var valuesOneArr = valuesOne.ToArray();
            var valuesTwoArr = valuesTwo.ToArray();
            fixed (double* ptrOne = valuesOneArr)
            fixed (double* ptrTwo = valuesTwoArr) {
                var ret = NativeMethods.covariance(ptrOne, (UIntPtr)valuesOneArr.Length, ptrTwo, (UIntPtr) valuesTwoArr.Length);

                if (ret == null)
                {
                    return null;
                }
                else
                {
                    return *ret;
                }
            }
        }
    }
}