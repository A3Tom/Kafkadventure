# Kafkadventure

On a quest to discover wit mad mysteries lurk in the depths of event streaming

## How to get started

### Requirements
- Visual Studio / msbuild
- Docker

### Languages involved
 - [X] C#
 - [X] Python
 - [X] Rust

### Gettin Started

Right this should be heavy sound. First off crack open a terminal n lets just get a wee Kafka container on the go using the `docker-compose.yml` file that's kickin about the base path

`docker compose up -d`

Now pick yer tech stack of choice

<details>
    <summary><h4>C# .Net 6</h4></summary>
    Don't be a shitebag; get the cli out  
    
    `cd .\dotnet`

    `dotnet build`  
    `dotnet watch --project KA.KafkaOrchestrator\KA.KafkaOrchestrator_Demo.csproj`
</details>

<details>
    <summary><h4>Python</h4></summary>
    Optional but recommended: create a new venv first  

    `cd .\python`

    `pip install kafka-python`  
    `dotnet watch --project KA.KafkaOrchestrator\KA.KafkaOrchestrator_Demo.csproj`
</details>
