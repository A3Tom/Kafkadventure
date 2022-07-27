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

        var subjectLine = BuildSubjectLine(animal, loch, hings);
        var sightingDto = BuildSightingDto(animal, loch, hings, "Auld pishy Tam");

        return new EventDto(Topic.SIGHTINGS, subjectLine, DateTime.UtcNow, sightingDto);
    }

    private static AnimalSightingDto BuildSightingDto(AnimalEnum animal, LochEnum loch, int hings, string reportedBy)
        => new(animal, loch, hings, reportedBy);

    public static string BuildSubjectLine(AnimalEnum animal, LochEnum loch, int hings)
        => $"{hings} {animal.ToString().ToLowerInvariant()}{(hings == 1 ? "" : "s")} {(hings == 1 ? "has" : "have")} been spotted near {loch.GetDescription()}";

    public record Request() : IRequest<EventDto>
    {
    }
}
