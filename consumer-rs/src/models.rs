use rdkafka::client::ClientContext;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::topic_partition_list::TopicPartitionList;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message as WarpMessage, Rejection};
use serde::{Serialize, Deserialize};

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

pub trait ToMapEventDto {
    fn to_event_dto(&self, title: String, count: i64) -> MapEventDto;
}

// TODO : Add a metadata object to this 
#[derive(Serialize, Deserialize, Debug)]
pub struct MapEventDto {
    pub event_id: String,
    pub title: String,
    pub count: i64,
    pub reported_by: String,
    pub latitude: f64,
    pub longitude: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SightingEventDto {
    pub event_id: String,
    pub animal: u64,
    pub loch: u64,
    pub count: u64,
    pub reported_by: String,
    pub latitude: f64,
    pub longitude: f64
}

impl ToMapEventDto for SightingEventDto{
    fn to_event_dto(&self, title: String, count: i64) -> MapEventDto {
        return MapEventDto { 
            title, 
            count,
            event_id: self.event_id.clone(), 
            reported_by: self.reported_by.clone(), 
            latitude: self.longitude,
            longitude: self.longitude 
        };
    }
}