using System;
using FinLib.Extensions;
using FinLib.Price;

namespace FinLib.Options;

public enum OptionType
{
    Call,
    Put,
}

public enum OptionStyle
{
    European,
    American,
}

public class OptionContract: IDisposable, IProfit<double>
{
    private readonly unsafe OptionContract_native* _handle;
    internal unsafe OptionContract_native* GetPtr() => _handle;

    public OptionContract(OptionType optionType, OptionStyle optionStyle, Side side, double strike, double premium)
    {
        unsafe
        {
            _handle = NativeMethods.option_contract_from(optionType.MapType(), optionStyle.MapStyle(), side.MapSide(), strike, premium);
        }
    }

    public double Payoff(double underlying)
    {
        unsafe
        {
            return NativeMethods.option_contract_payoff(_handle, underlying);
        }
    }

    public double Profit(double underlying)
    {
        unsafe
        {
            return NativeMethods.option_contract_profit(_handle, underlying);
        }
    }

    public bool WillBeExercised(double underlying)
    {
        unsafe
        {
            return NativeMethods.option_contract_will_be_exercised(_handle, underlying);
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.option_contract_destroy(_handle);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~OptionContract()
    {
        ReleaseUnmanagedResources();
    }
}