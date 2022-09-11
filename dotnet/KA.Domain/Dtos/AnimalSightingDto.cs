using KA.Domain.Enums;

namespace KA.Domain.Dtos;
public record AnimalSightingDto(Guid EventId, AnimalEnum Animal, LochEnum Loch, int Count, string ReportedBy, double Latitude, double Longitude);
