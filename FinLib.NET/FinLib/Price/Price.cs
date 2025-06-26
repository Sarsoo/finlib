using System;
using FinLib.Extensions;

namespace FinLib.Price;

public enum Side
{
    Buy, Sell
}

public class Price: IDisposable
{
    private readonly unsafe Price_native* _price;
    internal unsafe Price_native* GetPtr() => _price;

    internal unsafe Price(Price_native* price)
    {
        _price = price;
    }

    public Price(float value, Side side)
    {
        unsafe
        {
            _price = NativeMethods.price_new(value, side.MapSide());
        }
    }

    public Side Side
    {
        get
        {
            unsafe
            {
                return NativeMethods.price_get_side(_price).MapSide();
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.price_set_side(_price, value.MapSide());
            }
        }
    }

    public double Value
    {
        get
        {
            unsafe
            {
                return NativeMethods.price_get_val(_price);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.price_set_val(_price, value);
            }
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.price_destroy(_price);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~Price()
    {
        ReleaseUnmanagedResources();
    }
}