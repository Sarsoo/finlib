namespace FinLib.Test;
using FluentAssertions;

public class Tests
{
    [SetUp]
    public void Setup()
    {
    }

    [Test]
    public void TestCompoundInterest()
    {
        Interest.Interest.Compound(100, 0.05, 1, 1).Should().Be(105);
    }

    [Test]
    public void TestCompoundInterestMonthly()
    {
        Math.Round(Interest.Interest.Compound(100, 0.05, 1, 12), 2).Should().Be(105.12);
    }

    [Test]
    public void Covariance()
    {
        Stats.Stats.Covariance([1d, 2d, 3d, 4], [1d, 2, 3, 4]).Should().Be(1.6666666666666667);
    }

    [Test]
    public void CovarianceBreaking()
    {
        Stats.Stats.Covariance([1d, 2d, 3d, 4], [1d]).Should().BeNull();
    }
}