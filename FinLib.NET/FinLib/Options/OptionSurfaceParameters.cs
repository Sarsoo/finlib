using System;

namespace FinLib.Options;

public class OptionSurfaceParameters: IDisposable
{
    private readonly unsafe OptionSurfaceParameters_native* _handle;
    internal unsafe OptionSurfaceParameters_native* GetPtr() => _handle;

    public OptionSurfaceParameters(
        (nint, nint) underlyingPriceRange, (double, double) underlyingPriceMinMax,
        (nint, nint) strikePriceRange, (double, double) strikePriceMinMax,
        (nint, nint) volatilityRange, (double, double) volatilityMinMax,
        (nint, nint) riskFreeInterestRateRange, (double, double) riskFreeInterestRateMinMax,
        (nint, nint) dividendRange, (double, double) dividendMinMax,
        (nint, nint) timeToExpirationRange, (double, double) timeToExpirationMinMax)
    {
        unsafe
        {
            _handle = NativeMethods.option_surface_parameters_from(
                underlyingPriceRange.Item1, underlyingPriceRange.Item2, underlyingPriceMinMax.Item1, underlyingPriceMinMax.Item2,
                strikePriceRange.Item1, strikePriceRange.Item2, strikePriceMinMax.Item1, strikePriceMinMax.Item2,
                volatilityRange.Item1, volatilityRange.Item2, volatilityMinMax.Item1, volatilityMinMax.Item2,
                riskFreeInterestRateRange.Item1, riskFreeInterestRateRange.Item2, riskFreeInterestRateMinMax.Item1, riskFreeInterestRateMinMax.Item2,
                dividendRange.Item1, dividendRange.Item2, dividendMinMax.Item1, dividendMinMax.Item2,
                timeToExpirationRange.Item1, timeToExpirationRange.Item2, timeToExpirationMinMax.Item1, timeToExpirationMinMax.Item2
                );
        }
    }

    public OptionsSurface Walk()
    {
        unsafe
        {
            var surface = NativeMethods.option_surface_parameters_walk(_handle);
            return new OptionsSurface(surface);
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.option_surface_parameters_destroy(_handle);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~OptionSurfaceParameters()
    {
        ReleaseUnmanagedResources();
    }
}