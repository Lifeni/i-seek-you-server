use crate::{
    types::{Peer, PeerId, PeerIp, Response},
    PEER_IDS, PEER_MAP,
};
use serde_json::{json, Value};
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

pub fn gen_peer_id(sender: UnboundedSender<Message>) -> PeerId {
    let id = PEER_IDS.write().unwrap().pop_front().unwrap();
    let data = Response::Id {
        r#type: "id".to_string(),
        id: id.clone(),
    };
    sender.send(Message::text(json!(data).to_string())).unwrap();
    id
}

pub fn get_peer(
    peer_id: &String,
    sender: UnboundedSender<Message>,
) -> Option<(UnboundedSender<Message>, Peer)> {
    let map = PEER_MAP.read().unwrap();
    let (_, sender, peer) = match map.get(peer_id) {
        Some(peer) => {
            println!("[signaling] found peer {}", peer_id);
            peer.clone()
        }
        None => {
            let data = Response::Error {
                r#type: "error".to_string(),
                message: "No Peer Found".to_string(),
            };
            sender.send(Message::text(json!(data).to_string())).unwrap();
            println!("[signaling] not found peer {}", peer_id);
            return None;
        }
    };

    Some((sender, peer))
}

pub fn peer_sign(
    peer_id: &PeerId,
    peer_ip: &PeerIp,
    sender: UnboundedSender<Message>,
    message: Value,
) {
    let peer = Peer {
        id: peer_id.to_string(),
        name: message["name"].as_str().unwrap().to_string(),
        password: message["password"].as_bool().unwrap(),
        emoji: message["emoji"].as_str().unwrap().to_string(),
    };
    let mut map = PEER_MAP.write().unwrap();
    map.insert(peer_id.to_string(), (peer_ip.to_string(), sender, peer));
}

pub fn peer_call(self_id: &String, self_sender: UnboundedSender<Message>, message: Value) {
    let id = message["id"].as_str().unwrap().to_string();
    let map = PEER_MAP.read().unwrap();
    let (_, _, info) = map.get(self_id).unwrap();
    match get_peer(&id, self_sender.clone()) {
        Some((sender, _)) => {
            println!("[signaling] peer {} calls peer {}", self_id, id);
            sender
                .send(Message::text(
                    json!(Response::Call {
                        r#type: "call".to_string(),
                        peer: info.clone(),
                        password: message["password"]
                            .as_str()
                            .unwrap_or_else(|| "")
                            .to_string(),
                        pk: message["pk"].as_str().unwrap().to_string(),
                    })
                    .to_string(),
                ))
                .unwrap();
        }
        None => return,
    };
}

pub fn peer_find(self_sender: UnboundedSender<Message>, message: Value) {
    let id = message["id"].as_str().unwrap().to_string();
    match get_peer(&id, self_sender.clone()) {
        Some((_, peer)) => {
            self_sender
                .send(Message::text(
                    json!(Response::Peer {
                        r#type: "peer".to_string(),
                        peer: peer.clone(),
                    })
                    .to_string(),
                ))
                .unwrap();
        }
        None => return,
    };
}

pub fn peer_leave(peer_id: &PeerId) {
    PEER_IDS.write().unwrap().push_back(peer_id.to_string());
    PEER_MAP.write().unwrap().remove(peer_id);
}

/// Send lobby members of the same IP
pub fn peer_boardcast(peer_ip: &PeerIp) {
    let lobby = PEER_MAP.read().unwrap();
    let (mut senders, mut peers) = (Vec::new(), Vec::new());
    for (_, (ip, sender, peer)) in lobby.iter() {
        if ip == peer_ip {
            senders.push(sender.clone());
            peers.push(peer.clone());
        }
    }

    let response = Response::Lobby {
        r#type: "lobby".to_string(),
        peers,
    };
    let response = json!(&response).to_string();
    for sender in senders {
        sender.send(Message::text(&response)).unwrap();
    }
}
