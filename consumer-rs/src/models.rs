use rdkafka::client::ClientContext;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::topic_partition_list::TopicPartitionList;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message as WarpMessage, Rejection};

pub struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, _: &Rebalance) {}
    fn post_rebalance(&self, _: &Rebalance) {}
    fn commit_callback(&self, _: KafkaResult<()>, _: &TopicPartitionList) {}
}

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<WarpMessage, warp::Error>>>,
}
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
pub type Result<T> = std::result::Result<T, Rejection>;

// A type alias with your custom consumer can be created for convenience.
pub type LoggingConsumer = StreamConsumer<CustomContext>;