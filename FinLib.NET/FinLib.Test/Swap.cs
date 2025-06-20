using FinLib.Price;
using FluentAssertions;

namespace FinLib.Test;

public class SwapTest
{
    [Test]
    public void TestSwap()
    {
        using var swap = new Swap.Swap(100, Side.Buy, 0);

        swap.Payoff(100).Should().Be(0);
    }

    [Test]
    public void TestSwap2()
    {
        using var swap = new Swap.Swap(100, Side.Buy, 0);

        swap.Payoff(110).Should().Be(10);
    }

    [Test]
    public void TestSwapMultiple()
    {
        using var swap = new Swap.Swap(100, Side.Buy, 0);

        swap.Payoff([100d, 110d, 120d]).Should().Be(10);
    }
}