using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using FinLib.Extensions;
using FinLib.Price;

namespace FinLib.Swap;

public class Swap: IDisposable, IProfit<double>, IProfit<IEnumerable<double>>
{
    private readonly unsafe Swap_native* _swap;
    internal unsafe Swap_native* GetPtr() => _swap;

    public Swap(double fixedRate, Side fixedSide, double premium)
    {
        unsafe
        {
            _swap = NativeMethods.swap_from(fixedRate, fixedSide.MapSide(), premium);
        }
    }

    public double Payoff(double floatingRate)
    {
        unsafe
        {
            return NativeMethods.swap_payoff(_swap, floatingRate);
        }
    }

    public double Payoff(IEnumerable<double> values)
    {
        unsafe
        {
            var v = values.ToArray();
            fixed (double* valuesPtr = v){
                return NativeMethods.swap_payoff_from_multiple(_swap, valuesPtr, (UIntPtr)v.Length);
            }
        }
    }

    public double Profit(double floatingRate)
    {
        unsafe
        {
            return NativeMethods.swap_profit(_swap, floatingRate);
        }
    }

    public double Profit(IEnumerable<double> values)
    {
        unsafe
        {
            var v = values.ToArray();
            fixed (double* valuesPtr = v){
                return NativeMethods.swap_profit_from_multiple(_swap, valuesPtr, (UIntPtr)v.Length);
            }
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.swap_destroy(_swap);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~Swap()
    {
        ReleaseUnmanagedResources();
    }
}