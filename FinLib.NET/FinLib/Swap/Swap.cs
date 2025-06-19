using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using FinLib.Price;

namespace FinLib.Swap;

public class Swap: IDisposable
{
    private readonly unsafe Swap_native* _swap;
    internal unsafe Swap_native* GetPtr() => _swap;

    public Swap(double fixedRate, Side fixedSide, double premium)
    {
        unsafe
        {
            _swap = NativeMethods.swap_from(fixedRate, Price.Price.MapSide(fixedSide), premium);
        }
    }

    public double NetReturn(double floatingRate)
    {
        unsafe
        {
            return NativeMethods.swap_net_return(_swap, floatingRate);
        }
    }

    public double NetReturn(IEnumerable<double> values)
    {
        unsafe
        {
            var v = values.ToArray();
            fixed (double* valuesPtr = v){
                return NativeMethods.swap_net_return_from_multiple(_swap, valuesPtr, (UIntPtr)v.Length);
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