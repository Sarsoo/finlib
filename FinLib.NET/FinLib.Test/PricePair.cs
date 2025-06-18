using FinLib.Price;
using FluentAssertions;

namespace FinLib.Test;

public class PricePairTest
{
    [Test]
    public void TestPricePairCreation()
    {
        using var price = new PricePair(5, 7);

        price.Bid.Should().Be(5);
        price.Offer.Should().Be(7);
    }

    [Test]
    public void TestPricePairSpread()
    {
        using var price = new PricePair(5, 7);

        price.Spread.Should().Be(2);
    }

    [Test]
    public void TestPricePairMidpoint()
    {
        using var price = new PricePair(5, 7);

        price.Midpoint.Should().Be(6);
    }
}