using KA.Domain.Enums;
using Microsoft.Extensions.Logging;

namespace KA.Application.Services.Consumers.Sightings;
public class FrogSightingConsumer : SightingConsumerBase
{
    private ILogger<FrogSightingConsumer> _logger;

    public FrogSightingConsumer(ILogger<FrogSightingConsumer> logger) : base()
    {
        _logger = logger;
    }

    public override bool ConsumerCriteriaMet() => 
        _sighting.Animal == AnimalEnum.FROG;

    public override void Consume() => 
        _logger.LogInformation($"Amphibacious: {_messageReceived.Subject}");
}
