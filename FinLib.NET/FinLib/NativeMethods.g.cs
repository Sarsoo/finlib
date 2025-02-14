// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;


namespace FinLib
{
    internal static unsafe partial class NativeMethods
    {
        const string __DllName = "libfinlib_ffi";



        [DllImport(__DllName, EntryPoint = "interest_compound", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern double interest_compound(double principal, double rate, double time, double n);

        [DllImport(__DllName, EntryPoint = "covariance", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern double* covariance(double* arr, nuint len, double* arr_two, nuint len_two);

        [DllImport(__DllName, EntryPoint = "value_at_risk", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern double* value_at_risk(double* arr, nuint len, double confidence);


    }



}
