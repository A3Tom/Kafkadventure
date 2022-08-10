using KA.Domain.Enums;
using Microsoft.Extensions.Logging;

namespace KA.Application.Services.Consumers.Sightings;
public class SlothSightingConsumer : SightingConsumerBase
{
    private ILogger<SlothSightingConsumer> _logger;

    public SlothSightingConsumer(ILogger<SlothSightingConsumer> logger) : base()
    {
        _logger = logger;
    }

    public override bool ConsumerCriteriaMet() => 
        _sighting.Animal == AnimalEnum.SLOTH;

    public override void Consume() => 
        _logger.LogInformation($"Nae chance, {_messageReceived.Subject}");
}
