use anyhow::Result;
use std::net::{Ipv4Addr, UdpSocket};

pub fn build(addr: &str) -> Result<Client> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    socket.connect(addr)?;
    Ok(Client { socket })
}

pub struct Client {
    socket: UdpSocket,
}
