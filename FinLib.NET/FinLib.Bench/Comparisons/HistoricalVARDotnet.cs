namespace FinLib.Bench.Comparisons;

public static class HistoricalVARDotnet
{
    public static IEnumerable<(T, T)> Pairwise<T>(this IEnumerable<T> enumerable)
    {
        var previous = default(T);

        using (var e = enumerable.GetEnumerator())
        {
            if (e.MoveNext())
                previous = e.Current;

            while (e.MoveNext())
            {
                previous = e.Current;
                yield return (previous, e.Current);
            }
        }
    }

    public static IEnumerable<double> RatesOfChange(this IEnumerable<double> values)
    {
        foreach (var value in values.Pairwise())
        {
            yield return (value.Item2 - value.Item1) / value.Item1;
        }
    }

    public static double ValueAtRisk(List<double> values, double confidence)
    {
        var roc = values.RatesOfChange().Order().ToArray();
        var threshold = (int) Math.Floor(confidence * roc.Length);

        return roc[threshold];
    }
}