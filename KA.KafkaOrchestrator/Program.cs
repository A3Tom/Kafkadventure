using KA.Application.Services.Consumers.Sightings;
using KA.Application.Services.Producers;
using KA.Application.UseCases.GenerateNonsenseEvent;
using MediatR;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;

CreateHostBuilder(args).Build().Run();

static IHostBuilder CreateHostBuilder(string[] args) =>
    Host.CreateDefaultBuilder(args)
    .ConfigureServices((context, collection) =>
    {
        //collection.AddHostedService<DemoConsumer>();
        //collection.AddHostedService<DemoProducer>();
        collection.AddHostedService<FrogSightingConsumer>();
        collection.AddHostedService<DragonSightingConsumer>();
        collection.AddHostedService<SlothSightingConsumer>();
        collection.AddHostedService<SightingProducer>();

        collection.AddMediatR(typeof(GenerateNonsenseEvent));
    });