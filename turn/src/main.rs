use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::signal;
use tokio::time::Duration;
use turn::auth::*;
use turn::relay::relay_static::*;
use turn::server::{config::*, *};
use turn::Error;
use util::vnet::net::*;

pub mod configs;

struct MyAuthHandler {
    cred_map: HashMap<String, Vec<u8>>,
}

impl MyAuthHandler {
    fn new(cred_map: HashMap<String, Vec<u8>>) -> Self {
        MyAuthHandler { cred_map }
    }
}

impl AuthHandler for MyAuthHandler {
    fn auth_handle(
        &self,
        username: &str,
        _realm: &str,
        _src_addr: SocketAddr,
    ) -> Result<Vec<u8>, Error> {
        if let Some(pw) = self.cred_map.get(username) {
            Ok(pw.to_vec())
        } else {
            Err(Error::ErrFakeErr)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let configs = configs::get_configs();

    let mut cred_map = HashMap::new();
    let key = generate_auth_key(configs.username, configs.realm, configs.password);
    cred_map.insert(configs.username.to_string(), key);

    let conn = Arc::new(UdpSocket::bind(format!("0.0.0.0:{}", configs.port)).await?);
    println!("[turn] listening {}...", conn.local_addr()?);

    let server = Server::new(ServerConfig {
        conn_configs: vec![ConnConfig {
            conn,
            relay_addr_generator: Box::new(RelayAddressGeneratorStatic {
                relay_address: IpAddr::from_str(configs.public_ip)?,
                address: "0.0.0.0".to_owned(),
                net: Arc::new(Net::new(None)),
            }),
        }],
        realm: configs.realm.to_owned(),
        auth_handler: Arc::new(MyAuthHandler::new(cred_map)),
        channel_bind_timeout: Duration::from_secs(0),
    })
    .await?;

    println!("[turn] waiting for ctrl-c...");
    signal::ctrl_c()
        .await
        .expect("[turn] failed to listen for event");
    println!("\n[turn] closing connection now...");
    server.close().await?;

    Ok(())
}
