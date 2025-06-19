using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace FinLib.Risk;

public class Portfolio: IDisposable
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

    public void AddAsset(double portfolioWeight, string assetName, IEnumerable<double> values)
    {
        unsafe
        {
            var n = Encoding.UTF8.GetBytes(assetName);
            var v = values.ToArray();
            fixed (byte* namePtr = n)
            fixed (double* valuesPtr = v){
                NativeMethods.portfolio_add_asset(_portfolio, NativeMethods.portfolio_asset_new(portfolioWeight, namePtr, assetName.Length, valuesPtr, (UIntPtr)v.Length));
            }
        }
    }

    public bool ValidSize()
    {
        unsafe
        {
            return NativeMethods.portfolio_valid_sizes(_portfolio);
        }
    }

    public bool ValidWeights()
    {
        unsafe
        {
            return NativeMethods.portfolio_valid_weights(_portfolio);
        }
    }

    public bool IsValid()
    {
        unsafe
        {
            return NativeMethods.portfolio_is_valid(_portfolio);
        }
    }

    public void ApplyRatesOfChange()
    {
        unsafe
        {
            NativeMethods.portfolio_apply_rates_of_change(_portfolio);
        }
    }

    public (double, double)? GetMeanAndStdDev()
    {
        unsafe
        {
            var ret = NativeMethods.portfolio_get_mean_and_std(_portfolio);
            if (ret.is_valid)
            {
                return (ret.one, ret.two);
            }

            return null;
        }
    }

    public double? ValueAtRisk(double confidence, double initialInvestment)
    {
        unsafe
        {
            var ret = NativeMethods.portfolio_value_at_risk(_portfolio, confidence, initialInvestment);
            if (ret.is_valid)
            {
                return ret.val;
            }

            return null;
        }
    }

    public double? ValueAtRiskPercent(double confidence)
    {
        unsafe
        {
            var ret = NativeMethods.portfolio_value_at_risk_percent(_portfolio, confidence);
            if (ret.is_valid)
            {
                return ret.val;
            }

            return null;
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