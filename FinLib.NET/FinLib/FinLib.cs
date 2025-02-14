using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace FinLib.Stats;

public static class Stats
{
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