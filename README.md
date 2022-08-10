# Kafkadventure

On a quest to discover wit mad mysteries lurk in the depths of event streaming

## How to get started

### Requirements
- Visual Studio / msbuild
- Docker

### Languages involved
 - [X] C#
 - [X] Python
 - [ ] Rust

### Gettin Started

<details>
    <summary><h4>C# .Net 6</h4></summary>


    This should be heavy sound, first off get a wee Kafka container on the go

    `docker compose up -d`

    Compile the .Net solution n run it.
    Don't be a shitebag; get the cli out
    
    `cd .\dotnet`
    `dotnet build`  
    `dotnet watch --project KA.KafkaOrchestrator\KA.KafkaOrchestrator_Demo.csproj`
</details>
