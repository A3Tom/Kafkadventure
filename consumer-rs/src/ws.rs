use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, watch::Receiver};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};


pub async fn client_connection(ws: WebSocket, clients: Clients, mut rx: Receiver<String>) {
    println!("establishing client connection... {:?}", ws);
    let (client_ws_sender, client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));
    let uuid = Uuid::new_v4().to_string();
    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };

    clients.lock().await.insert(uuid.clone(), new_client);

    while rx.changed().await.is_ok() {
        let message: String;
        {
            let y = rx.borrow();
            message = y.to_owned();
        }

        client_msg("kafka", message.to_string(), &clients).await;
    }
    
    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn client_msg(client_id: &str, msg: String, clients: &Clients) {
    println!("received message from {}: {:}", client_id, msg);
}

async fn send_msg(client_id: &str, clients: &Clients) {
    let locked = clients.lock().await;
        match locked.get(client_id) {
            Some(v) => {
                if let Some(sender) = &v.sender {
                    println!("sending pong");
                    let _ = sender.send(Ok(Message::text("pong")));
                }
            }
            None => return,
        }
}