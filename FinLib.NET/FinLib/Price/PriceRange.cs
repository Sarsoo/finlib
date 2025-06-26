using System;

namespace FinLib.Price;

public class PriceRange: IDisposable
{
    private readonly unsafe PriceRange_native* _price;
    internal unsafe PriceRange_native* GetPtr() => _price;

    internal unsafe PriceRange(PriceRange_native* price)
    {
        _price = price;
    }

    public PriceRange(double open, double close, double high, double low, double volume)
    {
        unsafe
        {
            _price = NativeMethods.price_range_new(open, close, high, low, volume);
        }
    }


    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.price_range_destroy(_price);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~PriceRange()
    {
        ReleaseUnmanagedResources();
    }
}