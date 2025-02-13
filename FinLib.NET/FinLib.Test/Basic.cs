namespace FinLib.Test;
using FluentAssertions;

public class Tests
{
    [SetUp]
    public void Setup()
    {
    }

    [Test]
    public void TestAdd()
    {
        FinLib.Add(10, 10).Should().Be(20);
    }
}