namespace KA.Domain.Classes;
public static class SexantUtils
{
    private const int EARTH_RADIUS_KM = 6371;
    public static double CalculateGreatCircleDistance(double Lat1, double Long1, double Lat2, double Long2)
    {
        var latRad_1 = DegToRad(Lat1);
        var latRad_2 = DegToRad(Lat2);
        var longRad_1 = DegToRad(Long1);
        var longRad_2 = DegToRad(Long2);

        var deltaLat = latRad_2 - latRad_1;
        var deltaLong = longRad_2 - longRad_1;

        var havCentralAngle = CalculateHaversine(deltaLat, deltaLong, latRad_1, latRad_2);
        var centralAngleRad = CalculateCentralAngleRadians(havCentralAngle);

        return centralAngleRad * EARTH_RADIUS_KM;
    }

    private static double DegToRad(double d) => d * (Math.PI / 180);

    private static double CalculateHaversine(double deltaLat, double deltaLong, double latRad_1, double latRad_2)
        => 
        Math.Pow(
            Math.Sin(deltaLat / 2), 2)
            + (
                Math.Cos(latRad_1)
                * Math.Cos(latRad_2)
                * Math.Pow(Math.Sin(deltaLong / 2), 2)
            );

    private static double CalculateCentralAngleRadians(double havCentralAngle)
        =>
        2 * Math.Atan2(
                Math.Sqrt(havCentralAngle),
                Math.Sqrt(1 - havCentralAngle)
        );
}
