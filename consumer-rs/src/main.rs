// use clap::{App, Arg};
use log::{info, warn};

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Message as KafkaMessage, BorrowedMessage};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex, watch, watch::{Receiver, Sender}};
use warp::{ws::Message as WarpMessage, Filter, Rejection};

use crate::config::{Config, handle_config};
use crate::example_utils::setup_logger;

mod example_utils;
mod handlers;
mod ws;
mod config;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, _: &Rebalance) {
        // info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, _: &Rebalance) {
        // info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, _: KafkaResult<()>, _offsets: &TopicPartitionList) {
        // info!("Committing offsets: {:?}", result);
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<WarpMessage, warp::Error>>>,
}
type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Result<T> = std::result::Result<T, Rejection>;

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str], transmitter: Sender<String>) {
    print_kafka_version();

    let consumer: LoggingConsumer = build_kafka_consumer(brokers, group_id, topics);


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

fn build_kafka_consumer(brokers: &str, group_id: &str, topics: &[&str]) -> LoggingConsumer
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

fn output_kafka_message_to_websocket(tx: &Sender<String>, payload: &str, m :&BorrowedMessage)
{
    let _ = tx.send(format_output_message(payload, m));
}

fn format_output_message(payload: &str, m :&BorrowedMessage) -> String
{
    format!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
        m.key(), 
        payload, 
        m.topic(), 
        m.partition(), 
        m.offset(), 
        m.timestamp())
}

#[tokio::main]
async fn main() {
    let config: Config = handle_config();
    let brokers = format!("{}:{}",config.kafka_address, config.kafka_port);
    let group_id = &config.group_id;
    let topics = vec![config.topic.as_str()];
    let (tx, rx) = watch::channel("remo".to_string());

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and(with_receiver(rx))
        .and_then(handlers::ws_handler);
        
    let routes = ws_route.with(warp::cors().allow_any_origin());
    let websocket_task = async move {
        println!("Starting server");
        println!("Configuring websocket route");
        warp::serve(routes).run(([127, 0, 0, 1], 8000)).await
    };

    setup_logger(true, None);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(websocket_task);
    
    consume_and_print(&brokers, group_id, &topics, tx).await;
}

fn print_kafka_version()
{
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
}

fn with_receiver(rx: Receiver<String>) -> impl Filter<Extract = (Receiver<String>,), Error = Infallible> + Clone {
    warp::any().map(move || rx.clone())
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}