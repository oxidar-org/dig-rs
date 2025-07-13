use crate::packet::Domain;
use binrw::{BinRead, BinResult, BinWrite, binrw};
use std::{fmt::Display, net::Ipv4Addr};

#[derive(Clone, Debug, PartialEq)]
#[binrw]
#[brw(big)]
pub struct Route {
    pub domain: Domain,
    pub ttl: u32,
    pub adr: Address,
}

#[derive(Clone, Copy, Debug, PartialEq, BinRead, BinWrite)]
pub enum Address {
    Ipv4(
        #[br(parse_with = parse_ipv4)]
        #[bw(write_with = write_ipv4)]
        Ipv4Addr,
    ),
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::Ipv4(ip) => write!(f, "{ip}"),
        }
    }
}

#[binrw::parser(reader)]
fn parse_ipv4() -> BinResult<Ipv4Addr> {
    let len = u16::read_be(reader)?;
    if len != 4 {
        return Err(binrw::Error::AssertFail {
            pos: reader.stream_position()?,
            message: "Invalid IPv4 address length".to_string(),
        });
    }

    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(Ipv4Addr::from(buf))
}

#[binrw::writer(writer)]
fn write_ipv4(ip: &Ipv4Addr) -> BinResult<()> {
    4u16.write_be(writer)?;
    writer.write_all(&ip.octets())?;
    Ok(())
}
