using KA.Domain.Consts;
using KA.Domain.Dtos;
using KA.Domain.Enums;
using KA.Domain.Extensions;
using MediatR;

namespace KA.Application.UseCases.GenerateNonsenseEvent;

public class GenerateNonsenseEvent : IRequestHandler<GenerateNonsenseEvent.Request, EventDto>
{
    public Task<EventDto> Handle(Request request, CancellationToken cancellationToken) => 
        Task.Run(() => BuildRandomEvent());

    public static EventDto BuildRandomEvent()
    {
        var animal = (AnimalEnum)Random.Shared.Next(0, Enum.GetValues<AnimalEnum>().Length);
        var loch = (LochEnum)Random.Shared.Next(0, Enum.GetValues<LochEnum>().Length);
        var hings = Random.Shared.Next(0, Server.MAX_SIGHTINGS);

        var latitude = double.Parse($"{Random.Shared.Next(-90, 90)}.{Random.Shared.Next(0, 999999)}");
        var longitude = double.Parse($"{Random.Shared.Next(-180, 180)}.{Random.Shared.Next(0, 999999)}");

        var subjectLine = BuildSubjectLine(animal, loch, hings);
        var sightingDto = BuildSightingDto(animal, loch, hings, "Auld pishy Tam", latitude, longitude);

        return new EventDto(Topic.SIGHTINGS, subjectLine, DateTime.UtcNow, sightingDto);
    }

    private static AnimalSightingDto BuildSightingDto(AnimalEnum animal, LochEnum loch, int hings, string reportedBy, double latitude, double longitude)
        => new(Guid.NewGuid(), animal, loch, hings, reportedBy, latitude, longitude);

    public static string BuildSubjectLine(AnimalEnum animal, LochEnum loch, int hings)
        => $"{hings} {animal.ToString().ToLowerInvariant()}{(hings == 1 ? "" : "s")} {(hings == 1 ? "has" : "have")} been spotted near {loch.GetDescription()}";

    public record Request() : IRequest<EventDto>
    {
    }
}
