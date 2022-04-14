use crate::{
    handler::{gen_peer_id, peer_boardcast, peer_call, peer_find, peer_leave, peer_sign},
    types::{PeerId, PeerIp},
    PEER_MAP,
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

    let peer_id = gen_peer_id(sender.clone());
    let cloned_id = peer_id.clone();
    println!("Peer {} connected from {}", peer_id, peer_ip);

    tokio::task::spawn(async move {
        while let Some(message) = receiver.next().await {
            ws_sender
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("Websocket send error: {}", e);
                    peer_leave(&cloned_id)
                })
                .await;
        }
    });

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
    peer_boardcast(&peer_ip);
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
        "sign" => {
            peer_sign(peer_id, peer_ip, sender, message);
            peer_boardcast(peer_ip);
        }
        "find" => peer_find(sender, message),
        "call" => peer_call(peer_id, sender, message),
        "disconnect" => relay(message),
        "e2ee" | "answer" | "sdp" | "ice" | "error" => relay(message),
        "ping" => pong(sender),
        _ => unknown(),
    };
}

fn pong(sender: UnboundedSender<Message>) {
    sender
        .send(Message::text(json!({ "type": "pong" }).to_string()))
        .expect("Failed to send pong")
}

fn relay(message: Value) {
    let id = message["id"].as_str().unwrap().to_string();
    let r#type = message["type"].as_str().unwrap().to_string();
    let map = PEER_MAP.read().unwrap();
    match map.get(&id) {
        Some((_, sender, _)) => {
            println!("Relay {} message to {}", r#type, id);
            sender
                .send(Message::text(json!(message).to_string()))
                .unwrap();
        }
        None => (),
    };
}

fn unknown() {
    println!("Unknown message type")
}
