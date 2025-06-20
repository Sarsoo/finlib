using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace FinLib.Risk;

public static class ValueAtRisk
{
    public static double? Historical(IEnumerable<double> values, double confidence)
    {
        unsafe {
            var valueArr = values.ToArray();
            fixed (double* ptrOne = valueArr) {
                return NativeMethods.historical_value_at_risk(ptrOne, (UIntPtr)valueArr.Length, confidence);
            }
        }
    }

    public static double? VarCovar(IEnumerable<double> values, double confidence)
    {
        unsafe {
            var valueArr = values.ToArray();
            fixed (double* ptrOne = valueArr) {
                return NativeMethods.varcovar_value_at_risk(ptrOne, (UIntPtr)valueArr.Length, confidence);
            }
        }
    }

    public static double ScaleValueAtRisk(double initialValue, nint timeCycles) => NativeMethods.scale_value_at_risk(initialValue, timeCycles);
}