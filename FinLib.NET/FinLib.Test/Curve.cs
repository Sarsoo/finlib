using FinLib.Curve;
using FluentAssertions;

namespace FinLib.Test;

public class CurveTest
{
    [Test]
    public void TestCurveCreation()
    {
        using var curve = new Curve.Curve(CurveType.Differential);

        curve.Count.Should().Be(0);
        curve.Add(10, 10, DateTime.Today);
        curve.Count.Should().Be(1);
    }

    [Test]
    public void TestCurveRateRetrieval()
    {
        using var curve = new Curve.Curve(CurveType.Differential);

        curve.Add(10, 10, DateTime.Today);
        curve.CumulativeRateAt(DateTime.Today).Bid.Should().Be(10);
    }

    [Test]
    public void TestCurveRateRetrievalNull()
    {
        using var curve = new Curve.Curve(CurveType.Differential);

        curve.Add(10, 10, DateTime.Today);
        curve.RateAt(DateTime.Today + TimeSpan.FromDays(2)).Should().BeNull();
    }
}