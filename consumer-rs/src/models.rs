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

pub struct CustomContext;

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
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
pub type Result<T> = std::result::Result<T, Rejection>;

// A type alias with your custom consumer can be created for convenience.
pub type LoggingConsumer = StreamConsumer<CustomContext>;