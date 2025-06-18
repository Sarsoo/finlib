using System;
using FinLib.Price;

namespace FinLib.Curve;

public class Curve: IDisposable
{
    private readonly unsafe Curve_native* _curve;
    internal unsafe Curve_native* GetPtr() => _curve;

    public Curve()
    {
        unsafe
        {
            _curve = NativeMethods.curve_new();
        }
    }

    public nuint Count {
        get
        {
            unsafe
            {
                return NativeMethods.curve_size(_curve);
            }
        }
    }

    public void Add(double bid, double offer, DateTime date)
    {
        unsafe
        {
            NativeMethods.curve_add_rate_from(_curve, bid, offer, date.Year, Convert.ToUInt32(date.Month), Convert.ToUInt32(date.Day));
        }
    }

    public PricePair CumulativeRateAt(DateTime date)
    {
        unsafe
        {
            var item = NativeMethods.curve_get_cumulative_rate(_curve, date.Year, Convert.ToUInt32(date.Month), Convert.ToUInt32(date.Day));
            return new PricePair(item);
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.curve_destroy(_curve);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~Curve()
    {
        ReleaseUnmanagedResources();
    }
}