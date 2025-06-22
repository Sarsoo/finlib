using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using FinLib.Risk;

namespace FinLib.Portfolio;

public class Portfolio: IDisposable, IPayoff<double?>, IValueAtRisk
{
    private readonly unsafe Portfolio_native* _portfolio;
    internal unsafe Portfolio_native* GetPtr() => _portfolio;

    public Portfolio()
    {
        unsafe
        {
            _portfolio = NativeMethods.portfolio_new();
        }
    }

    public void AddAsset(string assetName, double quantity, IEnumerable<double> values)
    {
        unsafe
        {
            var n = Encoding.UTF8.GetBytes(assetName);
            var v = values.ToArray();
            fixed (byte* namePtr = n)
            fixed (double* valuesPtr = v){
                NativeMethods.portfolio_add_asset(_portfolio, NativeMethods.portfolio_asset_new(
                    // portfolioWeight,
                    namePtr, assetName.Length, quantity, valuesPtr, (UIntPtr)v.Length));
            }
        }
    }

    public void AddAsset(PortfolioAsset asset)
    {
        unsafe
        {
            NativeMethods.portfolio_add_asset(_portfolio, asset.GetPtr());
        }
    }

    public bool SizeValid
    {
        get
        {
            unsafe
            {
                return NativeMethods.portfolio_valid_sizes(_portfolio);
            }
        }
    }

    public nuint Count
    {
        get
        {
            unsafe
            {
                return NativeMethods.portfolio_size(_portfolio);
            }
        }
    }

    // public bool ValidWeights()
    // {
    //     unsafe
    //     {
    //         return NativeMethods.portfolio_valid_weights(_portfolio);
    //     }
    // }

    public bool IsValid
    {
        get
        {
            unsafe
            {
                return NativeMethods.portfolio_is_valid(_portfolio);
            }
        }
    }

    public void ApplyRatesOfChange()
    {
        unsafe
        {
            NativeMethods.portfolio_apply_rates_of_change(_portfolio);
        }
    }

    public (double, double)? MeanAndStdDev
    {
        get
        {
            unsafe
            {
                return NativeMethods.portfolio_get_mean_and_std(_portfolio);
            }
        }
    }

    public double? ValueAtRisk(double confidence, double? initialInvestment = null)
    {
        unsafe
        {
            return NativeMethods.portfolio_value_at_risk(_portfolio, confidence, initialInvestment);
        }
    }

    public double? ValueAtRiskPercent(double confidence)
    {
        unsafe
        {
            return NativeMethods.portfolio_value_at_risk_percent(_portfolio, confidence);
        }
    }

    public double? ValueAtRiskAfterTime(double confidence, nint at, double? initialInvestment = null)
    {
        unsafe
        {
            return NativeMethods.portfolio_value_at_risk_afer_time(_portfolio, confidence, initialInvestment, at);
        }
    }

    public double? ProfitLoss
    {
        get
        {
            unsafe
            {
                return NativeMethods.portfolio_profit_loss(_portfolio);
            }
        }
    }

    public double Payoff(double? underlying = null)
    {
        unsafe
        {
            return NativeMethods.portfolio_payoff(_portfolio, underlying);
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.portfolio_destroy(_portfolio);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~Portfolio()
    {
        ReleaseUnmanagedResources();
    }
}