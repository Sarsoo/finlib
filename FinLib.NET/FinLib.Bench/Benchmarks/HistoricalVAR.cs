using System.Security.Cryptography;
using BenchmarkDotNet.Attributes;
using FinLib.Bench.Comparisons;

namespace FinLib.Bench.Benchmarks;

public class HistoricalVar
{
    private const int N = 10000;

    // [Params(0.05d, 0.1d)]
    public double confidence = 0.05;
    [Params(10, 1000, 10000)]
    public int length;
    private double[] data;

    [GlobalSetup]
    public void Setup()
    {
        data = new double[length];
        var random = new Random(42);

        for (int i = 0; i < length; i++)
        {
            data[i] = random.NextDouble();
        }
    }

    [Benchmark]
    public double FinLibDotnet() => HistoricalVARDotnet.ValueAtRisk(data.ToList(), confidence);

    [Benchmark]
    public double FinLibRust() => FinLib.Risk.ValueAtRisk.Historical(data, confidence)!.Value;
}