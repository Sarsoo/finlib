using System;
using FinLib.Extensions;

namespace FinLib.Price;

public class PriceRangePair: IDisposable
{
    private readonly unsafe PriceRangePair_native* _price;
    internal unsafe PriceRangePair_native* GetPtr() => _price;

    internal unsafe PriceRangePair(PriceRangePair_native* price)
    {
        _price = price;
    }

    public PriceRangePair(DateTime open, DateTime close, PriceRange bid, PriceRange offer)
    {
        unsafe
        {
            _price = NativeMethods.price_range_pair_new(open.ToUnixTime(), close.ToUnixTime(), bid.GetPtr(), offer.GetPtr());
        }
    }


    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.price_range_pair_destroy(_price);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~PriceRangePair()
    {
        ReleaseUnmanagedResources();
    }
}