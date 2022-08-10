using KA.Domain.Enums;
using Microsoft.Extensions.Logging;

namespace KA.Application.Services.Consumers.Sightings;
public class LlamaSightingConsumer: SightingConsumerBase
{
    private ILogger<LlamaSightingConsumer> _logger;

    public LlamaSightingConsumer(ILogger<LlamaSightingConsumer> logger) : base()
    {
        _logger = logger;
    }

    public override bool ConsumerCriteriaMet() =>
        _sighting.Animal == AnimalEnum.LLAMA;

    public override void Consume() =>
        _logger.LogWarning($"Yasss man! {_messageReceived.Subject}");
}