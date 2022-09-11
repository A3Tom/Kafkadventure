use rdkafka::{config::{ClientConfig, RDKafkaLogLevel}, consumer::Consumer};
use rdkafka::consumer::CommitMode;
use rdkafka::message::{Message as KafkaMessage, BorrowedMessage};
use rdkafka::util::get_rdkafka_version;
use tokio::sync::watch::Sender;
use log::{info, warn};

use crate::models::{CustomContext, LoggingConsumer};

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
                
                output_kafka_message_to_websocket(&transmitter, payload, &m);
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

fn output_kafka_message_to_websocket(tx: &Sender<String>, payload: &str, m :&BorrowedMessage)
{
    let _ = tx.send(format_output_message(payload, m));
}

fn format_output_message(payload: &str, m :&BorrowedMessage) -> String
{
    format!("[{}] {}", m.topic(), payload)
}

fn print_kafka_version()
{
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
}