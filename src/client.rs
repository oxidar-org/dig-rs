use crate::packet::{Domain, Message, Route};
use anyhow::Result;
use rand::prelude::*;
use std::net::{Ipv4Addr, UdpSocket};

pub trait DNSResolver {
    fn query_aa(&self, domain: &str) -> Result<Vec<Route>>;
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
        let data: Vec<u8> = m.into_bytes();
        self.socket.send(&data)?;
        Ok(())
    }

    fn receive(&self) -> Result<Message> {
        let mut buffer = [0; 256];
        self.socket.recv(&mut buffer)?;
        Message::from_bytes(&buffer)
    }
}

impl DNSResolver for Client {
    fn query_aa(&self, domain: &str) -> Result<Vec<Route>> {
        let mut rng = rand::rng();
        let domain = Domain::new_aa(domain.try_into()?);
        let msg = Message::query_domain(rng.random(), domain);
        self.send(msg)?;
        let resp = self.receive()?;
        Ok(resp.answers().to_vec())
    }
}
