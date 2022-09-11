use rdkafka::{config::{ClientConfig, RDKafkaLogLevel}, consumer::Consumer};
use rdkafka::consumer::CommitMode;
use rdkafka::message::{Message as KafkaMessage};
use rdkafka::util::get_rdkafka_version;
use serde_json::Value;
use tokio::sync::watch::Sender;
use log::{info, warn};



use crate::models::{CustomContext, LoggingConsumer, MapEventDto};

pub fn build_kafka_consumer(brokers: &str, group_id: &str, topics: &[&str]) -> LoggingConsumer
{
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer.subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    return consumer
}

pub async fn consume_and_output(brokers: &str, group_id: &str, topics: &[&str], transmitter: Sender<String>) {
    let consumer: LoggingConsumer = build_kafka_consumer(brokers, group_id, topics);

    print_kafka_version();
    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                
                output_kafka_message_to_websocket(&transmitter, payload);
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

fn output_kafka_message_to_websocket(tx: &Sender<String>, payload: &str)
{
    let event_value: Value = serde_json::from_str(payload).unwrap();
    let map_event = value_to_event_dto(event_value);

    let _ = tx.send(format_output(map_event));
}

fn value_to_event_dto(event_value: Value) -> MapEventDto {
    return MapEventDto { 
        event_id: event_value["MeatNTatties"]["EventId"].to_string(), 
        title: event_value["Subject"].to_string(), 
        count: event_value["MeatNTatties"]["Count"].as_i64().unwrap_or_else(|| 0), 
        reported_by: event_value["MeatNTatties"]["ReportedBy"].to_string(),
        latitude: event_value["MeatNTatties"]["Latitude"].as_f64().unwrap_or_else(|| 0.0),
        longitude: event_value["MeatNTatties"]["Longitude"].as_f64().unwrap_or_else(|| 0.0)
    };
}

fn format_output(event_dto: MapEventDto) -> String{
    format!("{} | {} | {} | {} | {} by: {}", 
        event_dto.title,
        event_dto.count,
        event_dto.event_id,
        event_dto.latitude,
        event_dto.longitude,
        event_dto.reported_by
    )
}

fn print_kafka_version()
{
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
}