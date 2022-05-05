use rustun::server::{BindingHandler, UdpServer};
use std::net::{IpAddr, SocketAddr};
use trackable::error::MainError;

#[macro_use]
extern crate trackable;

struct Address {
    host: IpAddr,
    port: u16,
}

fn main() -> Result<(), MainError> {
    let address = Address {
        host: IpAddr::from([0, 0, 0, 0]),
        port: 7201,
    };

    println!("[stun] listening on {}:{}", address.host, address.port);

    let server = track!(fibers_global::execute(UdpServer::start(
        fibers_global::handle(),
        SocketAddr::new(address.host, address.port),
        BindingHandler
    )))?;

    track!(fibers_global::execute(server))?;

    Ok(())
}
