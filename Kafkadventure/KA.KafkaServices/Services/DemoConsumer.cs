using Kafka.Public;
using Kafka.Public.Loggers;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using System.Text;

namespace KA.Application.Services;
public class DemoConsumer : IHostedService
{
    private ILogger<DemoConsumer> _logger;
    private ClusterClient _cluster;

    public DemoConsumer(ILogger<DemoConsumer> logger)
    {
        _logger = logger;
        _cluster = new ClusterClient(new Configuration
        {
            Seeds = "localhost:9092"
        }, new ConsoleLogger());
    }

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        _cluster.ConsumeFromLatest("demo");
        _cluster.MessageReceived += record =>
        {
            _logger.LogInformation($"Received: {Encoding.UTF8.GetString((byte[])record.Value)}");
        };
    }

    public Task StopAsync(CancellationToken cancellationToken)
    {
        _cluster?.Dispose();

        return Task.CompletedTask;
    }
}
