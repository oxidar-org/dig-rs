use crate::packet::{Domain, Message};
use anyhow::Result;
use deku::prelude::*;
use rand::prelude::*;
use std::net::{Ipv4Addr, UdpSocket};

pub trait DNSResolver {
    fn query_aa(&self, domain: &str) -> Result<Vec<Ipv4Addr>>;
}

pub fn connect(addr: &str) -> Result<impl DNSResolver> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    socket.connect(addr)?;
    Ok(Client { socket })
}

struct Client {
    socket: UdpSocket,
}

impl Client {
    fn send(&self, m: Message) -> Result<()> {
        let data: Vec<u8> = m.to_bytes()?;
        self.socket.send(&data)?;
        Ok(())
    }

    fn receive(&self) -> Result<Message> {
        let mut buffer = [0; 256];
        self.socket.recv(&mut buffer)?;
        let (_, msg) = Message::from_bytes((&buffer, 0))?;
        Ok(msg)
    }
}

impl DNSResolver for Client {
    fn query_aa(&self, domain: &str) -> Result<Vec<Ipv4Addr>> {
        let mut rng = rand::rng();
        let domain = Domain::new_aa(domain.try_into()?);
        let msg = Message::query_domain(rng.random(), domain);
        self.send(msg)?;
        let resp = self.receive()?;
        Ok(resp.answers().iter().filter_map(|a| a.ipv4()).collect())
    }
}
