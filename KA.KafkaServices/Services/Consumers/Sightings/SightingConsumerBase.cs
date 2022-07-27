using KA.Domain.Consts;
using KA.Domain.Dtos;
using Kafka.Public;
using Kafka.Public.Loggers;
using Microsoft.Extensions.Hosting;
using System.Text;
using System.Text.Json;

namespace KA.Application.Services.Consumers.Sightings;
public abstract class SightingConsumerBase : IHostedService
{
    private protected ClusterClient _cluster;

    private protected EventDto _messageReceived;
    private protected AnimalSightingDto _sighting;

    public SightingConsumerBase()
    {
        _cluster = BuildClusterClient();
    }

    private static ClusterClient BuildClusterClient() => new ( 
            new Configuration { Seeds = Server.SEED_SERVER }, 
            new ConsoleLogger()
        );

    public abstract void Consume();
    public abstract bool ConsumerCriteriaMet();

    private bool BaseCriteriaMet() =>
        _sighting?.Count > 1;

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        _cluster.ConsumeFromLatest(Topic.SIGHTINGS);

        _cluster.MessageReceived += HandleMessageReceived();
    }

    private Action<RawKafkaRecord> HandleMessageReceived()
    {
        return record =>
        {
            if (record is null)
                return;

            _messageReceived = JsonSerializer.Deserialize<EventDto>(Encoding.UTF8.GetString((byte[])record.Value));
            _sighting = JsonSerializer.Deserialize<AnimalSightingDto>(_messageReceived.MeatNTatties.ToString());

            if (BaseCriteriaMet() && ConsumerCriteriaMet())
                Consume();
        };
    }

    public Task StopAsync(CancellationToken cancellationToken)
    {
        _cluster?.Dispose();

        return Task.CompletedTask;
    }
}
