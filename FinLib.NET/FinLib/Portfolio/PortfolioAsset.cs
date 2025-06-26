using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using FinLib.Extensions;
using FinLib.Price;
using FinLib.Risk;

namespace FinLib.Portfolio;

public class PortfolioAsset: IDisposable, IPayoff<double?>, IProfit<double?>, IValueAtRisk
{
    private readonly unsafe PortfolioAsset_native* _handle;
    internal unsafe PortfolioAsset_native* GetPtr() => _handle;

    public PortfolioAsset(string assetName, double quantity, TimeSpan timeSpan)
    {
        unsafe
        {
            var n = Encoding.UTF8.GetBytes(assetName);
            fixed (byte* namePtr = n){
                _handle = NativeMethods.portfolio_asset_new(
                    namePtr, assetName.Length, quantity, timeSpan.MapTimeSpan());
            }
        }
    }

     public PriceRangePair? CurrentValue
     {
         get
         {
             unsafe
             {
                 var item = NativeMethods.portfolio_asset_current_value(_handle);
                 if (item is not null)
                 {
                     return new PriceRangePair(item);
                 }

                 return null;
             }
         }
     }

    public double? CurrentTotalValue
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

    public double Payoff(double? underlying = null)
    {
        unsafe
        {
            return NativeMethods.portfolio_asset_payoff(_handle, underlying);
        }
    }

    public double Profit(double? underlying = null)
    {
        unsafe
        {
            return NativeMethods.portfolio_asset_profit(_handle, underlying);
        }
    }

    public double? ValueAtRisk(double confidence, double? initialInvestment = null)
    {
        unsafe
        {
            return NativeMethods.portfolio_asset_value_at_risk(_handle, confidence, initialInvestment);
        }
    }

    public double? ValueAtRiskPercent(double confidence)
    {
        unsafe
        {
            return NativeMethods.portfolio_asset_value_at_risk_percent(_handle, confidence);
        }
    }

    public double? ValueAtRiskAfterTime(double confidence, nint at, double? initialInvestment = null)
    {
        unsafe
        {
            return NativeMethods.portfolio_asset_value_at_risk_afer_time(_handle, confidence, initialInvestment, at);
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