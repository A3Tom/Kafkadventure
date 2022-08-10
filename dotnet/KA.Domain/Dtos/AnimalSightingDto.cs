using KA.Domain.Enums;

namespace KA.Domain.Dtos;
public record AnimalSightingDto(AnimalEnum Animal, LochEnum Loch, int Count, string ReportedBy);
