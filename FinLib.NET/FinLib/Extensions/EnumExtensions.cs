using System;
using FinLib.Curve;
using FinLib.Options;
using FinLib.Price;
using TimeSpan = FinLib.Portfolio.TimeSpan;

namespace FinLib.Extensions;

internal static class EnumExtensions
{
    internal static CurveType_native MapType(this CurveType side) => side == CurveType.Absolute ? CurveType_native.Absolute : CurveType_native.Differential;
    internal static CurveType MapType(this CurveType_native side) => side == CurveType_native.Absolute ? CurveType.Absolute : CurveType.Differential;

    internal static OptionType_native MapType(this OptionType type) => type == OptionType.Call ? OptionType_native.Call : OptionType_native.Put;
    internal static OptionType MapType(this OptionType_native type) => type == OptionType_native.Call ? OptionType.Call : OptionType.Put;

    internal static OptionStyle_native MapStyle(this OptionStyle type) => type == OptionStyle.European ? OptionStyle_native.European : OptionStyle_native.American;
    internal static OptionStyle MapStyle(this OptionStyle_native type) => type == OptionStyle_native.European ? OptionStyle.European : OptionStyle.American;

    internal static TimeSpan MapTimeSpan(this TimeSpan_native t) => t switch
    {
        TimeSpan_native.Second => TimeSpan.Second,
        TimeSpan_native.Minute => TimeSpan.Minute,
        TimeSpan_native.Hourly => TimeSpan.Hourly,
        TimeSpan_native.Daily => TimeSpan.Daily,
        TimeSpan_native.Weekly => TimeSpan.Weekly,
        _ => throw new ArgumentOutOfRangeException(nameof(t), t, null)
    };
    internal static TimeSpan_native MapTimeSpan(this TimeSpan t) => t switch
    {
        TimeSpan.Second => TimeSpan_native.Second,
        TimeSpan.Minute => TimeSpan_native.Minute,
        TimeSpan.Hourly => TimeSpan_native.Hourly,
        TimeSpan.Daily => TimeSpan_native.Daily,
        TimeSpan.Weekly => TimeSpan_native.Weekly,
        _ => throw new ArgumentOutOfRangeException(nameof(t), t, null)
    };

    internal static Side_native MapSide(this Side side) => side == Side.Buy ? Side_native.Buy : Side_native.Sell;
    internal static Side MapSide(this Side_native side) => side == Side_native.Buy ? Side.Buy : Side.Sell;
}