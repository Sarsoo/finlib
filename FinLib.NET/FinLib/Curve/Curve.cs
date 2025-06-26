using System;
using FinLib.Extensions;
using FinLib.Price;

namespace FinLib.Curve;

public enum CurveType
{
    Absolute, Differential
}

public class Curve: IDisposable
{
    private readonly unsafe Curve_native* _curve;
    internal unsafe Curve_native* GetPtr() => _curve;

    public Curve(CurveType type)
    {
        unsafe
        {
            _curve = NativeMethods.curve_new(type.MapType());
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

    public PricePair? RateAt(DateTime date)
    {
        unsafe
        {
            var item = NativeMethods.curve_get_rate(_curve, date.Year, Convert.ToUInt32(date.Month), Convert.ToUInt32(date.Day));
            if (item is null)
            {
                return null;
            }
            else
            {
                return new PricePair(item);
            }
        }
    }

    public PricePair? CumulativeRateAt(DateTime date)
    {
        unsafe
        {
            var item = NativeMethods.curve_get_cumulative_rate(_curve, date.Year, Convert.ToUInt32(date.Month), Convert.ToUInt32(date.Day));
            if (item is null)
            {
                return null;
            }
            else
            {
                return new PricePair(item);
            }
        }
    }

    public PricePair? AbsoluteRateAt(DateTime date)
    {
        unsafe
        {
            var item = NativeMethods.curve_get_absolute_rate(_curve, date.Year, Convert.ToUInt32(date.Month), Convert.ToUInt32(date.Day));
            if (item is null)
            {
                return null;
            }
            else
            {
                return new PricePair(item);
            }
        }
    }

    public PricePair CarryRate(DateTime from, DateTime to)
    {
        unsafe
        {
            var item = NativeMethods.curve_get_carry_rate(_curve,
                from.Year, Convert.ToUInt32(from.Month), Convert.ToUInt32(from.Day),
                to.Year, Convert.ToUInt32(to.Month), Convert.ToUInt32(to.Day)
                );
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