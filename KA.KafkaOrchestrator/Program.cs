using KA.Application.Services.Consumers;
using KA.Application.Services.Producers;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;

CreateHostBuilder(args).Build().Run();

static IHostBuilder CreateHostBuilder(string[] args) =>
    Host.CreateDefaultBuilder(args)
    .ConfigureServices((context, collection) =>
    {
        collection.AddHostedService<DemoConsumer>();
        collection.AddHostedService<DemoProducer>();
    });