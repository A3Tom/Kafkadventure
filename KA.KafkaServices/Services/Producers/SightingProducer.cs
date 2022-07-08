using Confluent.Kafka;
using KA.Application.UseCases.GenerateNonsenseEvent;
using KA.Domain.Dtos;
using MediatR;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using System.Text.Json;

namespace KA.Application.Services.Producers;
public class SightingProducer : IHostedService
{
    private ILogger<SightingProducer> _logger;
    private IProducer<Null, string> _producer;

    private readonly IMediator _mediator;

    public SightingProducer(ILogger<SightingProducer> logger, IMediator mediator)
    {
        _logger = logger;
        var config = new ProducerConfig()
        {
            BootstrapServers = "localhost:9092"
        };
        _producer = new ProducerBuilder<Null, string>(config).Build();
        _mediator = mediator;
    }

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        while (true)
        {
            var newEvent = await _mediator.Send(new GenerateNonsenseEvent.Request());
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
