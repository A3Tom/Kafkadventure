use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, watch::Receiver};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};


#[allow(unused_variables)]
pub async fn client_connection(ws: WebSocket, clients: Clients, mut rx: Receiver<String>) {
    println!("establishing client connection... {:?}", ws);
    let (client_ws_sender, _) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));
    let client_id = Uuid::new_v4().to_string();
    let new_client = Client {
        client_id: client_id.clone(),
        sender: Some(client_sender),
    };

    println!("Client [{}] added", &client_id);
    clients.lock().await.insert(client_id.clone(), new_client);

    while rx.changed().await.is_ok() {
        let msg = rx.borrow().to_owned().to_string();
        forward_kafka_message(msg, &clients, &client_id).await;

        // if(client disconnects)
        // {
        //     clients.lock().await.remove(&client_id);
        //     println!("client [{}] disconnected", client_id);
        // }
    }
    
    
}

async fn forward_kafka_message(msg: String, clients: &Clients, client_id: &str) {
    let locked = clients.lock().await;
    match locked.get(client_id) {
        Some(v) => {
            if let Some(sender) = &v.sender {
                println!("notifiying socket client {} of new message from kafka", client_id);
                let _ = sender.send(Ok(Message::text(&msg)));
            }
        }
        None => return,
    }
}
