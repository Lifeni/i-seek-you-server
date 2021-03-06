use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

pub type PeerId = String;
pub type PeerIp = String;

pub type PeerInfo = (PeerIp, UnboundedSender<Message>, Peer);
pub type Peers = HashMap<PeerId, PeerInfo>;

pub type PeerMap = Arc<RwLock<Peers>>;
pub type PeerIds = Arc<RwLock<VecDeque<PeerId>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: PeerId,
    pub emoji: String,
    pub name: String,
    pub password: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Ok {
        r#type: String,
    },
    Error {
        r#type: String,
        message: String,
    },
    Id {
        r#type: String,
        id: PeerId,
    },
    Lobby {
        r#type: String,
        peers: Vec<Peer>,
    },
    Peer {
        r#type: String,
        peer: Peer,
    },
    Call {
        r#type: String,
        peer: Peer,
        password: String,
        pk: String,
    },
}
