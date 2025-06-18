using System;

namespace FinLib.Price;

public class PricePair: IDisposable
{
    private readonly unsafe PricePair_native* _price;
    internal unsafe PricePair_native* GetPtr() => _price;

    internal unsafe PricePair(PricePair_native* price)
    {
        _price = price;
    }

    public PricePair(float bid, float offer)
    {
        unsafe
        {
            _price = NativeMethods.price_pair_new(bid, offer);
        }
    }

    public double Bid
    {
        get
        {
            unsafe
            {
                return NativeMethods.price_pair_get_bid(_price);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.price_pair_set_bid(_price, value);
            }
        }
    }

    public double Offer
    {
        get
        {
            unsafe
            {
                return NativeMethods.price_pair_get_offer(_price);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.price_pair_set_offer(_price, value);
            }
        }
    }

    public double Spread {
        get
        {
            unsafe
            {
                return NativeMethods.price_pair_spread(_price);
            }
        }
    }

    public double Midpoint {
        get
        {
            unsafe
            {
                return NativeMethods.price_pair_midpoint(_price);
            }
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.price_pair_destroy(_price);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~PricePair()
    {
        ReleaseUnmanagedResources();
    }
}