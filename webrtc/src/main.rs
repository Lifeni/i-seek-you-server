/* #[macro_use]
extern crate trackable;

use fibers_global::{execute, handle};
use rustun::channel::Channel;
use rustun::client::Client as StunClient;
use rustun::message::Request;
use rustun::transport::StunUdpTransporter;
use rusturn::auth::AuthParams;
use rusturn::client::UdpClient;
use rusturn::transport::UdpOverTurnTransporter;
use std::net::SocketAddr;
use stun_codec::rfc5389;
use stun_codec::{MessageDecoder, MessageEncoder};

struct AuthInfo {
    turn_server: SocketAddr,
    stun_server: SocketAddr,
    username: String,
    password: String,
}

fn main() -> Result<(), trackable::error::MainError> {
    let auth_info = AuthInfo {
        turn_server: SocketAddr::from(([0, 0, 0, 0], 8082)),
        stun_server: SocketAddr::from(([0, 0, 0, 0], 8083)),
        username: "you".to_string(),
        password: "password".to_string(),
    };

    let auth_params = track!(AuthParams::new(&auth_info.username, &auth_info.password))?;

    let turn_client = track!(execute(UdpClient::allocate(
        auth_info.turn_server,
        auth_params
    )))?;
    let transporter =
        UdpOverTurnTransporter::<_, MessageEncoder<_>, MessageDecoder<_>>::new(turn_client);

    let stun_channel = Channel::new(StunUdpTransporter::new(transporter));
    let stun_client = StunClient::new(&handle(), stun_channel);

    let request = Request::<rfc5389::Attribute>::new(rfc5389::methods::BINDING);
    let response = track!(execute(stun_client.call(auth_info.stun_server, request)))?;
    println!("{:?}", response);

    Ok(())
}
 */

#[macro_use]
extern crate trackable;

use rustun::server::{BindingHandler, UdpServer};
use trackable::error::MainError;

#[derive(Debug)]
struct Args {
    port: u16,
}

fn main() -> Result<(), MainError> {
    let args = Args { port: 8082 };
    let addr = track_any_err!(format!("0.0.0.0:{}", args.port).parse())?;

    let server = track!(fibers_global::execute(UdpServer::start(
        fibers_global::handle(),
        addr,
        BindingHandler
    )))?;
    track!(fibers_global::execute(server))?;
    Ok(())
}
