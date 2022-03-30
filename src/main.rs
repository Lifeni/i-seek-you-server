use std::{
    collections::{HashMap, VecDeque},
    env,
    io::Error as IoError,
    net::{IpAddr, SocketAddr},
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use rand::seq::SliceRandom;
use rand::thread_rng;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

#[derive(Debug, Clone)]
struct Sender {
    ip: IpAddr,
    tx: UnboundedSender<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Peer {
    id: String,
    emoji: String,
    name: String,
    password: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Response {
    Lobby { r#type: String, data: Vec<Peer> },
}

type Peers = HashMap<String, (Sender, Peer)>;
type PeerMap = Arc<Mutex<Peers>>;
type PeerIds = Arc<Mutex<VecDeque<String>>>;

/// Generate peer id vector,
/// e.g. `["0000", "0001", ..., "9999"]`
fn create_peer_ids() -> PeerIds {
    let mut ids = (0..9999)
        .map(|x: i32| -> String { format!("{:0>4}", x.to_string()) })
        .collect::<Vec<String>>();
    ids.shuffle(&mut thread_rng());
    PeerIds::new(Mutex::new(VecDeque::from(ids)))
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    peer_map: PeerMap,
    peer_ids: PeerIds,
) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let peer_ip = addr.ip();
    let peer_id = peer_ids.lock().unwrap().pop_front().expect("No more ids");

    let (sender, receiver) = unbounded();
    let (outgoing, incoming) = ws_stream.split();

    // Send peer id to the client
    sender
        .unbounded_send(Message::text(
            json!({ "type": "id", "data": peer_id }).to_string(),
        ))
        .unwrap();

    // Send lobby members of the same IP
    let update_lobby = |peer_map: Peers| {
        let lobby = peer_map
            .iter()
            .filter(|(_, (sender, _))| sender.ip == peer_ip)
            .map(|(_, (_, peer))| peer.clone())
            .collect();

        let response = Response::Lobby {
            r#type: "lobby".to_string(),
            data: lobby,
        };
        let response = serde_json::to_string(&response).unwrap();
        for (_, (sender, _)) in peer_map.iter() {
            sender
                .tx
                .unbounded_send(Message::text(response.clone()))
                .unwrap();
        }
    };

    let broadcast_incoming = incoming.try_for_each(|msg| {
        let message = &msg.to_text().unwrap();
        if !message.is_empty() {
            let message: Value = serde_json::from_str(message).expect("Error parsing message");

            match message["type"].as_str().unwrap() {
                // Receive peer info from the client
                "hello" => {
                    let data = (
                        Sender {
                            ip: peer_ip,
                            tx: sender.clone(),
                        },
                        Peer {
                            id: peer_id.clone(),
                            name: message["name"].as_str().unwrap().to_string(),
                            password: message["password"].as_bool().unwrap(),
                            emoji: message["emoji"].as_str().unwrap().to_string(),
                        },
                    );
                    // Save peer info and notify other peers
                    let mut peer_map = peer_map.lock().unwrap();
                    peer_map.insert(peer_id.clone(), data.clone());
                    update_lobby(peer_map.clone());
                }
                // Ping message from the client
                "ping" => sender
                    .unbounded_send(Message::Text(json!({ "type": "pong" }).to_string()))
                    .expect("Failed to send pong"),
                // Any other message
                _ => println!("Unknown message type"),
            }
        }

        future::ok(())
    });

    let receive_from_others = receiver.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("#{} - {} disconnected", peer_id, &addr);
    let mut peer_map = peer_map.lock().unwrap();
    peer_map.remove(&peer_id);
    peer_ids.lock().unwrap().push_back(peer_id);
    update_lobby(peer_map.clone());
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8081".to_string());

    let peer_map = PeerMap::new(Mutex::new(HashMap::new()));
    let peer_ids = create_peer_ids();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            stream,
            addr,
            peer_map.clone(),
            peer_ids.clone(),
        ));
    }

    Ok(())
}
