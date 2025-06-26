using System;
using FinLib.Extensions;

namespace FinLib.Price;

public class PriceTimestamp: IDisposable
{
    private readonly unsafe PriceTimestamp_native* _price;
    internal unsafe PriceTimestamp_native* GetPtr() => _price;

    internal unsafe PriceTimestamp(PriceTimestamp_native* price)
    {
        _price = price;
    }

    public PriceTimestamp(double value, Side side, DateTime timestampMillis)
    {
        unsafe
        {
            _price = NativeMethods.price_timestamp_new(value, side.MapSide(), timestampMillis.ToUnixTime());
        }
    }


    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.price_timestamp_destroy(_price);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~PriceTimestamp()
    {
        ReleaseUnmanagedResources();
    }
}