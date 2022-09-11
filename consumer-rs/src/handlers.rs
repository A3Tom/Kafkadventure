use crate::{ws, models::{Clients, Result}};
use tokio::sync::watch::Receiver;
use warp::Reply;

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients, rx: Receiver<String>) -> Result<impl Reply> {
    println!("ws_handler");
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients, rx)))
}