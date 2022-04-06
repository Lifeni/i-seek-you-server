use crate::model::{Response};
use crate::PEER_MAP;
use serde_json::json;
use warp::ws::Message;

/// Send lobby members of the same IP
pub fn send_lobby(peer_ip: &String) {
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
