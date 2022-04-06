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
    Lobby { r#type: String, peers: Vec<Peer> },
}
