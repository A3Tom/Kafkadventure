using KA.Domain.Classes;
using Shouldly;

namespace KA.KafkaOrchestrator.Domain.Tests.Classes;
public class HaversineCalculatorTests
{
    [Theory]
    [InlineData(45, 120, 48, 125, 507.49)]
    [InlineData(52.52, 13.4050, 48.8566, 2.3522, 877.46)]
    public void Hings(double lat_1, double long_1, double lat_2, double long_2, double expected)
    {
        var actual = HaversineCalculator.CalculateGreatCircleDistance(lat_1, long_1, lat_2, long_2);

        Math.Round(actual, 2).ShouldBe(expected);
    }
}
