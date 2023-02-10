# Kafkadventure

On a quest to discover wit mad mysteries lurk in the depths of event streaming

## How to get started

### Requirements
- Visual Studio / msbuild
- Docker

### Tech Stack Involved
 - [ ] React
 - [X] Python
 - [X] C#
 - [X] Rust
 - [X] Kafka
 - [X] MongoDB
 - [] ElasticSearch

### Gettin Started

Right this should be heavy sound. First off crack open a terminal n lets just get a wee Kafka container on the go using the `docker-compose.yml` file that's kickin about the base path

`docker compose up -d`

Now pick yer tech stack of choice

<details>
    <summary><h4>C# .Net 6</h4></summary>
    Don't be a shitebag; get the cli out  
    
    cd .\dotnet
    dotnet watch --project KA.KafkaOrchestrator\KA.KafkaOrchestrator_Demo.csproj
</details>

<details>
    <summary><h4>Rust</h4></summary>
    Get those sockets spoutin

    cd .\consumer-rs

    cargo build
    cargo run
</details>

<details>
    <summary><h4>Python</h4></summary>
    Optional but recommended: create a new venv first

    cd .\python

    pip install kafka-python
    py omni_listener.py
</details>

### Todo list
- Build a web interface for the web socket messages
  - Parse the lat/long on a map
- Create a better getting started mechanism
- Actually handle disconnects from the socket listeners
- Find a better use for python in this project
- Generate geofenced coordinates for events