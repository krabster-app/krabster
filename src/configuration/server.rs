use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfiguration {
    pub listen_address: IpAddr,
    pub listen_port: u16,
}

const LISTEN_ADDRESS_DEFAULT: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
const LISTEN_PORT_DEFAULT: u16 = 3000;

impl Default for ServerConfiguration {
    fn default() -> Self {
        Self {
            listen_address: LISTEN_ADDRESS_DEFAULT,
            listen_port: LISTEN_PORT_DEFAULT,
        }
    }
}
