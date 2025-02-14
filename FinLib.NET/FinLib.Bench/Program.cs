using BenchmarkDotNet.Running;
using FinLib.Bench.Benchmarks;

namespace FinLib.Bench
{
    public class Program
    {
        public static void Main(string[] args)
        {
            var summary = BenchmarkRunner.Run<HistoricalVar>();
        }
    }
}