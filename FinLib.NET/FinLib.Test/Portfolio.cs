using FinLib.Risk;
using FluentAssertions;
using TimeSpan = FinLib.Portfolio.TimeSpan;

namespace FinLib.Test;

public class PortfolioTest
{
    [Test]
    public void TestPortfolioCreation()
    {
        using var portfolio = new Portfolio.Portfolio();
        portfolio.AddAsset("first", 1, TimeSpan.Second);
        portfolio.AddAsset("second", 1, TimeSpan.Second);

        var (mean, std) = portfolio.MeanAndStdDev!.Value;
        mean.Should().Be(0);
        std.Should().Be(0);
    }

    [Test]
    public void TestPortfolioValid()
    {
        var portfolio = new Portfolio.Portfolio();
        portfolio.AddAsset("first", 1, TimeSpan.Second);
        portfolio.AddAsset("second", 1, TimeSpan.Second);

        portfolio.IsValid.Should().BeFalse();

        var portfolio2 = new Portfolio.Portfolio();
        portfolio2.AddAsset("first", 1, TimeSpan.Second);
        portfolio2.AddAsset("second", 1, TimeSpan.Second);

        portfolio2.IsValid.Should().BeTrue();
    }
}