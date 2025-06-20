using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace FinLib.Risk;

public class PortfolioAsset: IDisposable, IPayoff<double?>
{
    private readonly unsafe PortfolioAsset_native* _handle;
    internal unsafe PortfolioAsset_native* GetPtr() => _handle;

    public PortfolioAsset(string assetName, double quantity, IEnumerable<double> values)
    {
        unsafe
        {
            var n = Encoding.UTF8.GetBytes(assetName);
            var v = values.ToArray();
            fixed (byte* namePtr = n)
            fixed (double* valuesPtr = v){
                _handle = NativeMethods.portfolio_asset_new(
                    namePtr, assetName.Length, quantity, valuesPtr, (UIntPtr)v.Length);
            }
        }
    }

    public void ApplyRatesOfChange()
    {
        unsafe
        {
            NativeMethods.portfolio_asset_apply_rates_of_change(_handle);
        }
    }

    public double CurrentValue
    {
        get
        {
            unsafe
            {
                return NativeMethods.portfolio_asset_current_value(_handle);
            }
        }
    }

    public double CurrentTotalValue
    {
        get
        {
            unsafe
            {
                return NativeMethods.portfolio_asset_current_total_value(_handle);
            }
        }
    }

    public double? ProfitLoss
    {
        get
        {
            unsafe
            {
                return NativeMethods.portfolio_asset_profit_loss(_handle);
            }
        }
    }

    public double Payoff(double? underlying)
    {
        unsafe
        {
            return NativeMethods.portfolio_asset_payoff(_handle, underlying);
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.portfolio_asset_destroy(_handle);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~PortfolioAsset()
    {
        ReleaseUnmanagedResources();
    }
}