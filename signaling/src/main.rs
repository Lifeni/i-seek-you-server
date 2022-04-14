use crate::signaling::handle_signaling;
use crate::types::{PeerId, PeerIds, PeerMap};
use rand::{prelude::SliceRandom, thread_rng};
use std::{
    collections::VecDeque,
    env,
    net::{IpAddr, SocketAddr},
    sync::RwLock,
};
use warp::Filter;
use warp_real_ip::get_forwarded_for;

mod handler;
mod signaling;
mod types;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PEER_IDS: PeerIds = create_peer_ids();
    static ref PEER_MAP: PeerMap = PeerMap::default();
}

#[tokio::main]
async fn main() {
    let port = env::args().nth(1).unwrap_or_else(|| "8081".to_string());

    let signaling = warp::path::end()
        .and(warp::ws())
        .and(warp::addr::remote())
        .and(get_forwarded_for())
        .map(
            |ws: warp::ws::Ws, remote_ip: Option<SocketAddr>, peer_ips: Vec<IpAddr>| {
                let peer_ip = match peer_ips.first() {
                    Some(ip) => ip.to_string(),
                    None => remote_ip.unwrap().ip().to_string(),
                };

                ws.on_upgrade(move |socket| handle_signaling(socket, peer_ip))
            },
        );

    warp::serve(signaling)
        .run(([0, 0, 0, 0], port.parse().expect("Invalid port")))
        .await;
}

/// Generate peer id vector,
/// e.g. `["0000", "0001", ..., "9999"]`
pub fn create_peer_ids() -> PeerIds {
    let mut ids = (0..9999)
        .map(|x: i32| -> PeerId { format!("{:0>4}", x.to_string()) })
        .collect::<Vec<PeerId>>();
    ids.shuffle(&mut thread_rng());
    PeerIds::new(RwLock::new(VecDeque::from(ids)))
}
