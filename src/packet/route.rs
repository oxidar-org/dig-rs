use deku::{ctx::Order, prelude::*};
use std::{fmt::Display, net::Ipv4Addr};

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Route {
    _tbi: u16,
    ttl: u32,
    #[deku(
        reader = "Address::read(deku::reader)",
        writer = "Address::write(deku::writer, &self.adr)"
    )]
    adr: Address,
}

impl Route {
    pub fn ipv4(&self) -> Option<Ipv4Addr> {
        match self.adr {
            Address::Ipv4(ip) => Some(ip),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Address {
    Ipv4(Ipv4Addr),
}

impl Address {
    fn read<R: std::io::Read + std::io::Seek>(
        r: &mut deku::reader::Reader<R>,
    ) -> Result<Self, DekuError> {
        let mut octets = [0u8; 4];
        r.read_bytes(4, &mut octets, Order::Msb0)?;
        Ok(Address::Ipv4(Ipv4Addr::from(octets)))
    }

    fn write<W: std::io::Write + std::io::Seek>(
        w: &mut Writer<W>,
        a: &Self,
    ) -> Result<(), DekuError> {
        match a {
            Address::Ipv4(ip) => w.write_bytes(&ip.octets()),
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::Ipv4(ip) => write!(f, "{ip}"),
        }
    }
}
