use rusturn::auth::AuthParams;
use rusturn::server::UdpServer;
use std::net::{IpAddr, SocketAddr};
use uuid::Uuid;

#[macro_use]
extern crate trackable;

#[derive(Debug)]
struct Option {
    username: String,
    password: String,
    realm: String,
    nonce: String,
}

struct Address {
    host: IpAddr,
    port: u16,
}

fn main() -> Result<(), trackable::error::MainError> {
    let uuid = Uuid::new_v4().to_string();
    let address = Address {
        host: IpAddr::from([0, 0, 0, 0]),
        port: 7202,
    };
    let option = Option {
        username: "webrtc".to_string(),
        password: "webrtc".to_string(),
        realm: "default".to_string(),
        nonce: uuid,
    };

    println!("[turn] listening on {}:{}", address.host, address.port);
    println!("[turn] option: {:#?}", option);

    let turn_server = track!(fibers_global::execute(UdpServer::start(
        SocketAddr::new(address.host, address.port),
        AuthParams::with_realm_and_nonce(
            &option.username,
            &option.password,
            &option.realm,
            &option.nonce
        )
        .unwrap(),
    )))?;

    track!(fibers_global::execute(turn_server))?;

    Ok(())
}
