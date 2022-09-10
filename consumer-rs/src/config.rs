use std::env;

#[derive(Debug)]
pub struct Config {
    pub kafka_address: String,
    pub kafka_port: String,
    pub topic: String,
    pub group_id: String,
    pub ws_host: String,
    pub ws_port: String,
}

/// Collects the used environment variables.
pub fn handle_config() -> Config {
    let kafka_address = env::var("KAFKA_HOST").unwrap_or_else(|_| "localhost".to_string());
    let kafka_port = env::var("KAFKA_HOST_PORT").unwrap_or_else(|_| "9092".to_string());
    let topic = env::var("KAFKA_TOPIC").unwrap_or_else(|_| "sightings".to_string());
    let group_id = env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "consumer-rs-1".to_string());
    // websocket environment
    let ws_host = env::var("WS_HOST").unwrap_or_else(|_| "localhost".to_string());
    let ws_port = env::var("WS_PORT").unwrap_or_else(|_| "8000".to_string());

    Config {
        kafka_address,
        kafka_port,
        topic,
        group_id,
        ws_host,
        ws_port
    }
}