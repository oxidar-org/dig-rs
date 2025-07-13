use crate::packet::Domain;
use binrw::{BinRead, BinResult, BinWrite};
use std::{fmt::Display, net::Ipv4Addr};

#[derive(Clone, Debug, PartialEq, BinRead, BinWrite)]
#[brw(big)]
pub struct Route {
    pub domain: Domain,
    pub ttl: u32,
    pub adr: Address,
}

#[derive(Clone, Copy, Debug, PartialEq, BinRead, BinWrite)]
#[brw(big)]
pub enum Address {
    #[brw(magic(4u16))]
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
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(Ipv4Addr::from(buf))
}

#[binrw::writer(writer)]
fn write_ipv4(ip: &Ipv4Addr) -> BinResult<()> {
    writer.write_all(&ip.octets())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_address_parse() {
        let chunk = &[0x00, 0x04, 0x7f, 0x00, 0x00, 0x01];
        let adr = Address::read(&mut Cursor::new(chunk)).unwrap();
        assert_eq!(adr.to_string(), "127.0.0.1".to_string());
    }

    #[test]
    fn test_address_write() {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let adr = Address::Ipv4(ip);
        let mut writer = Cursor::new(Vec::new());
        adr.write(&mut writer).unwrap();

        assert_eq!(writer.into_inner(), &[0x00, 0x04, 0x7f, 0x00, 0x00, 0x01]);
    }
}
