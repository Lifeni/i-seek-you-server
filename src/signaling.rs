use crate::{
    lobby::lobby_broadcast,
    model::{PeerId, PeerIp},
    peer::{get_peer_id, peer_join, peer_leave},
};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use serde_json::{json, Value};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

pub async fn handle_signaling(ws: WebSocket, peer_ip: PeerIp) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (sender, receiver) = mpsc::unbounded_channel();
    let mut receiver = UnboundedReceiverStream::new(receiver);

    tokio::task::spawn(async move {
        while let Some(message) = receiver.next().await {
            ws_sender
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("Websocket send error: {}", e);
                })
                .await;
        }
    });

    let peer_id = get_peer_id(sender.clone());
    println!("Peer {} connected from {}", peer_id, peer_ip);

    while let Some(result) = ws_receiver.next().await {
        let message = match result {
            Ok(message) => message,
            Err(e) => {
                eprintln!("Websocket error: {}", e);
                break;
            }
        };
        if let Ok(message) = message.to_str() {
            handle_message(sender.clone(), &peer_ip, &peer_id, message).await;
        };
    }

    peer_leave(&peer_id);
    lobby_broadcast(&peer_ip);
}

pub async fn handle_message(
    sender: UnboundedSender<Message>,
    peer_ip: &PeerIp,
    peer_id: &PeerId,
    message: &str,
) {
    let message: Value = serde_json::from_str(message).expect("Error parsing message");
    match message["type"].as_str().unwrap() {
        // Receive peer info from the client
        "hi" => {
            peer_join(peer_id, peer_ip, sender, message);
            lobby_broadcast(peer_ip);
        }
        "ping" => pong(sender),
        _ => unknown(),
    }
}

fn pong(sender: UnboundedSender<Message>) {
    sender
        .send(Message::text(json!({ "type": "pong" }).to_string()))
        .expect("Failed to send pong")
}

fn unknown() {
    println!("Unknown message type")
}
