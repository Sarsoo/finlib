using System;

namespace FinLib.Price;

public enum Side
{
    Buy, Sell
}

public class Price: IDisposable
{
    private readonly unsafe Price_native* _price;
    internal unsafe Price_native* GetPtr() => _price;

    private Side_native MapSide(Side side) => side == Side.Buy ? Side_native.Buy : Side_native.Sell;
    private Side MapSide(Side_native side) => side == Side_native.Buy ? Side.Buy : Side.Sell;

    internal unsafe Price(Price_native* price)
    {
        _price = price;
    }

    public Price(float value, Side side)
    {
        unsafe
        {
            _price = NativeMethods.price_new(value, MapSide(side));
        }
    }

    public Side Side
    {
        get
        {
            unsafe
            {
                return MapSide(NativeMethods.price_get_side(_price));
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.price_set_side(_price, MapSide(value));
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