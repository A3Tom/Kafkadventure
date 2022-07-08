using KA.Domain.Enums;
using Microsoft.Extensions.Logging;

namespace KA.Application.Services.Consumers.Sightings;
public class DragonSightingConsumer : SightingConsumerBase
{
    private ILogger<DragonSightingConsumer> _logger;

    public DragonSightingConsumer(ILogger<DragonSightingConsumer> logger) : base()
    {
        _logger = logger;
    }

    public override bool ConsumerCriteriaMet() => 
        _sighting.Animal == AnimalEnum.DRAGON;

    public override void Consume() => 
        _logger.LogInformation($"Here, fuck that; {_messageReceived.Subject}");
}
