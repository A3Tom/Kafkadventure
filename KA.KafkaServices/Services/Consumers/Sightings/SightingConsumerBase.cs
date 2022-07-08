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
        _cluster = new ClusterClient(new Configuration
        {
            Seeds = "localhost:9092"
        }, new ConsoleLogger());
    }

    public abstract void Consume();
    public abstract bool ConsumerCriteriaMet();

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        _cluster.ConsumeFromLatest(Topics.SIGHTINGS);

        _cluster.MessageReceived += HandleMessageReceived();
    }

    private Action<RawKafkaRecord> HandleMessageReceived()
    {
        return record =>
        {
            _messageReceived = JsonSerializer.Deserialize<EventDto>(Encoding.UTF8.GetString((byte[])record.Value));
            _sighting = JsonSerializer.Deserialize<AnimalSightingDto>(_messageReceived.MeatNTatties.ToString());

            if (ConsumerCriteriaMet())
                Consume();
        };
    }

    public Task StopAsync(CancellationToken cancellationToken)
    {
        _cluster?.Dispose();

        return Task.CompletedTask;
    }
}
