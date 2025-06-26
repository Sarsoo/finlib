using System;

namespace FinLib.Extensions;

public static class HelperExtensions
{
    public static long ToUnixTime(this DateTime date)
    {
        return (long) date.ToUniversalTime().Subtract(
            new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc)
        ).TotalMilliseconds;
    }
}