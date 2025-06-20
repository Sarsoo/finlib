using System;
using FinLib.Price;

namespace FinLib.Options;

public enum OptionType
{
    Call,
    Put,
}

public class OptionStrategyComponent: IDisposable, IProfit<double>
{
    private readonly unsafe OptionStrategyComponent_native* _handle;
    internal unsafe OptionStrategyComponent_native* GetPtr() => _handle;

    internal static OptionType_native MapType(OptionType type) => type == OptionType.Call ? OptionType_native.Call : OptionType_native.Put;
    internal static OptionType MapType(OptionType_native type) => type == OptionType_native.Call ? OptionType.Call : OptionType.Put;

    public OptionStrategyComponent(OptionType optionType, Side side, double strike, double premium)
    {
        unsafe
        {
            _handle = NativeMethods.option_strategy_component_from(MapType(optionType), Price.Price.MapSide(side), strike, premium);
        }
    }

    public double Payoff(double underlying)
    {
        unsafe
        {
            return NativeMethods.option_strategy_component_payoff(_handle, underlying);
        }
    }

    public double Profit(double underlying)
    {
        unsafe
        {
            return NativeMethods.option_strategy_component_profit(_handle, underlying);
        }
    }

    public bool WillBeExercised(double underlying)
    {
        unsafe
        {
            return NativeMethods.option_strategy_component_will_be_exercised(_handle, underlying);
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.option_strategy_component_destroy(_handle);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~OptionStrategyComponent()
    {
        ReleaseUnmanagedResources();
    }
}