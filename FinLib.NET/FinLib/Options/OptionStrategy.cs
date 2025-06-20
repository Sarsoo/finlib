using System;

namespace FinLib.Options;

public class OptionStrategy: IDisposable, IProfit<double>
{
    private readonly unsafe OptionStrategy_native* _handle;
    internal unsafe OptionStrategy_native* GetPtr() => _handle;

    public OptionStrategy()
    {
        unsafe
        {
            _handle = NativeMethods.option_strategy_new();
        }
    }

    public nuint Size
    {
        get
        {
            unsafe
            {
                return NativeMethods.option_strategy_size(_handle);
            }
        }
    }

    public void Add(OptionStrategyComponent component)
    {
        unsafe
        {
            NativeMethods.option_strategy_add_component(_handle, component.GetPtr());
        }
    }

    public double Payoff(double underlying)
    {
        unsafe
        {
            return NativeMethods.option_strategy_payoff(_handle, underlying);
        }
    }

    public double Profit(double underlying)
    {
        unsafe
        {
            return NativeMethods.option_strategy_profit(_handle, underlying);
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.option_strategy_destroy(_handle);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~OptionStrategy()
    {
        ReleaseUnmanagedResources();
    }
}