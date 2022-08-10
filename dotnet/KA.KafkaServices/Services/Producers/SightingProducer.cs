using Confluent.Kafka;
using KA.Application.UseCases.GenerateNonsenseEvent;
using KA.Domain.Consts;
using MediatR;
using Microsoft.Extensions.Hosting;
using System.Text.Json;

namespace KA.Application.Services.Producers;
public class SightingProducer : IHostedService
{
    private IProducer<Null, string> _producer;

    private readonly IMediator _mediator;

    public SightingProducer(IMediator mediator)
    {
        _mediator = mediator ?? throw new ArgumentNullException(nameof(mediator));

        var config = BuildProducerConfig();
        _producer = new ProducerBuilder<Null, string>(config).Build();
    }

    private ProducerConfig BuildProducerConfig() => new() { BootstrapServers = Server.SEED_SERVER };

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        while (true)
        {
            var newEvent = await _mediator.Send(new GenerateNonsenseEvent.Request(), cancellationToken);

            await _producer.ProduceAsync(newEvent.Topic, new Message<Null, string>()
            {
                Value = JsonSerializer.Serialize(newEvent)
            }, cancellationToken);

            _producer.Flush(TimeSpan.FromSeconds(2));
            Thread.Sleep(1800);
        }
    }

    public Task StopAsync(CancellationToken cancellationToken)
    {
        _producer?.Dispose();

        return Task.CompletedTask;
    }
}
