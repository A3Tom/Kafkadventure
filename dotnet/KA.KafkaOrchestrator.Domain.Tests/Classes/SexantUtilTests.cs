using KA.Domain.Classes;
using Shouldly;

namespace KA.KafkaOrchestrator.Domain.Tests.Classes;
public class SexantUtilTests
{
    [Theory]
    [InlineData(45, 120, 48, 125, 507.49)]
    [InlineData(-45, 120, 48, 125, 10352.62)]
    [InlineData(45, -120, 48, 125, 7895.15)]
    [InlineData(45, -120, -48, 125, 15178.74)]
    [InlineData(52.52, 13.4050, 48.8566, 2.3522, 877.46)]
    public void GivenKnownExamples_ShouldReturnExpectedDistanceTo2SigFig_WhenValidCoordinates(double lat_1, double long_1, double lat_2, double long_2, double expected)
    {
        var actual = SexantUtils.CalculateGreatCircleDistance(lat_1, long_1, lat_2, long_2);

        Math.Round(actual, 2).ShouldBe(expected);
    }

    [Theory]
    [InlineData(45, 120, 48, 125, 047.13)]
    [InlineData(52.52, 13.4050, 48.8566, 2.3522, 246.74)]
    public void CalculateInitialBearing_ShouldReturnExpectedBearing_WhenValidCoordinates(double lat_1, double long_1, double lat_2, double long_2, double expected)
    {
        var actual = SexantUtils.CalculateInitialBearing(lat_1, long_1, lat_2, long_2);

        Math.Round(actual, 2).ShouldBe(expected);
    }

    [Theory]
    [InlineData(45, 120, 48, 125, 050.76)]
    [InlineData(52.52, 13.4050, 48.8566, 2.3522, 238.18)]
    public void CalculateFinalBearing_ShouldReturnExpectedBearing_WhenValidCoordinates(double lat_1, double long_1, double lat_2, double long_2, double expected)
    {
        var actual = SexantUtils.CalculateFinalBearing(lat_1, long_1, lat_2, long_2);

        Math.Round(actual, 2).ShouldBe(expected);
    }
}
