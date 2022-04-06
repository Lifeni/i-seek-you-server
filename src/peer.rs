use crate::{
    model::{Peer, PeerId, PeerIp},
    PEER_IDS, PEER_MAP,
};
use serde_json::{Value, json};
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

pub fn get_peer_id(sender: UnboundedSender<Message>) -> PeerId {
    let peer_id = PEER_IDS.write().unwrap().pop_front().unwrap();
    sender
        .send(Message::text(
            json!({ "type": "id", "id": peer_id }).to_string(),
        ))
        .unwrap();
    peer_id
}

pub fn peer_join(
    peer_id: &PeerId,
    peer_ip: &PeerIp,
    sender: UnboundedSender<Message>,
    message: Value,
) {
    PEER_MAP.write().unwrap().insert(
        peer_id.to_string(),
        (
            peer_ip.to_string(),
            sender,
            Peer {
                id: peer_id.to_string(),
                name: message["name"].as_str().unwrap().to_string(),
                password: message["password"].as_bool().unwrap(),
                emoji: message["emoji"].as_str().unwrap().to_string(),
            },
        ),
    );

    println!("Peer {} connected from {}", peer_id, peer_ip);
}

pub fn peer_leave(peer_id: &PeerId) {
    PEER_IDS.write().unwrap().push_back(peer_id.to_string());
    PEER_MAP.write().unwrap().remove(peer_id);
}
