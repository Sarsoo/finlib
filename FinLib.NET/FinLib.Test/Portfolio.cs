using FluentAssertions;

namespace FinLib.Test;

public class PortfolioTest
{
    [Test]
    public void TestPortfolioCreation()
    {
        using var portfolio = new Portfolio();
        portfolio.AddAsset(0.5, "first", [0.5, 0.5, 0.5, 0.5]);
        portfolio.AddAsset(0.5, "second", [0.5, 0.5, 0.5, 0.5]);

        var (mean, std) = portfolio.GetMeanAndStdDev()!.Value;
        mean.Should().Be(0);
        std.Should().Be(0);
    }

    [Test]
    public void TestPortfolioValid()
    {
        var portfolio = new Portfolio();
        portfolio.AddAsset(0.5, "first", [0.5, 0.5, 0.5, 0.5]);
        portfolio.AddAsset(0.5, "second", [0.5, 0.5, 0.5]);

        portfolio.IsValid().Should().BeFalse();

        var portfolio2 = new Portfolio();
        portfolio2.AddAsset(0.5, "first", [0.5, 0.5, 0.5, 0.5]);
        portfolio2.AddAsset(0.5, "second", [0.5, 0.5, 0.5, 0.5]);

        portfolio2.IsValid().Should().BeTrue();
    }
}