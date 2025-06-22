using System;
using FinLib.Options;

namespace FinLib.Portfolio;

public class Strategy: IDisposable, IProfit<double>
{
    private readonly unsafe Strategy_native* _handle;
    internal unsafe Strategy_native* GetPtr() => _handle;

    public Strategy()
    {
        unsafe
        {
            _handle = NativeMethods.strategy_new();
        }
    }

    public nuint Size
    {
        get
        {
            unsafe
            {
                return NativeMethods.strategy_size(_handle);
            }
        }
    }

    public void Add(IProfit<double> component)
    {
        unsafe {
            if (component is OptionContract oc){
                NativeMethods.strategy_add_option_component(_handle, oc.GetPtr());
            }
            else if (component is Swap.Swap s)
            {
                NativeMethods.strategy_add_swap_component(_handle, s.GetPtr());
            }
            else
            {
                throw new NotImplementedException();
            }
        }
    }

    public double Payoff(double underlying)
    {
        unsafe
        {
            return NativeMethods.strategy_payoff(_handle, underlying);
        }
    }

    public double Profit(double underlying)
    {
        unsafe
        {
            return NativeMethods.strategy_profit(_handle, underlying);
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.strategy_destroy(_handle);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~Strategy()
    {
        ReleaseUnmanagedResources();
    }
}