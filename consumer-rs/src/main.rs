use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{Mutex, watch, watch::Receiver};
use models::Clients;
use warp::Filter;

use crate::config::{Config, handle_config};
use crate::logger::setup_logger;
use crate::kafka::consume_and_output;

mod logger;
mod handlers;
mod ws;
mod config;
mod models;
mod kafka;

#[tokio::main]
async fn main() {
    let config: Config = handle_config();
    let brokers = format!("{}:{}",config.kafka_address, config.kafka_port);
    let group_id = &config.group_id;
    let topics = vec![config.topic.as_str()];
    let (tx, rx) = watch::channel("remo".to_string());

    let websocket_task = async move {
        println!("Configuring websocket routes");

        let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
        let ws_route = warp::path(config.ws_path)
            .and(warp::ws())
            .and(with_clients(clients.clone()))
            .and(with_receiver(rx))
            .and_then(handlers::ws_handler);
        let routes = ws_route.with(warp::cors().allow_any_origin());

        println!("Starting server");
        warp::serve(routes).run((config.ws_host, config.ws_port)).await
    };

    setup_logger(true, None);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(websocket_task);
    
    consume_and_output(&brokers, group_id, &topics, tx).await;
}

fn with_receiver(rx: Receiver<String>) -> impl Filter<Extract = (Receiver<String>,), Error = Infallible> + Clone {
    warp::any().map(move || rx.clone())
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}