using Confluent.Kafka;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;

namespace KA.Application.Services.Producers;
public class DemoProducer : IHostedService
{
    private ILogger<DemoProducer> _logger;
    private IProducer<Null, string> _producer;

    public DemoProducer(ILogger<DemoProducer> logger)
    {
        _logger = logger;
        var config = new ProducerConfig()
        {
            BootstrapServers = "localhost:9092"
        };
        _producer = new ProducerBuilder<Null, string>(config).Build();
    }

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        for (int i = 1; i <= 20; ++i)
        {
            var value = $"Fanny B{string.Join("", Enumerable.Repeat("a", i))}ws";
            _logger.LogInformation(value);

            await _producer.ProduceAsync("demo", new Message<Null, string>()
            {
                Value = value
            }, cancellationToken);
        }

        _producer.Flush(TimeSpan.FromSeconds(10));
    }

    public Task StopAsync(CancellationToken cancellationToken)
    {
        _producer?.Dispose();

        return Task.CompletedTask;
    }
}
