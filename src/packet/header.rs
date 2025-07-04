use deku::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct PacketId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum QueryMode {
    #[deku(id = "0")]
    Query,
    #[deku(id = "1")]
    Response,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, DekuRead, DekuWrite)]
pub struct OpCode(#[deku(bits = "4")] u8);

#[derive(Clone, Copy, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum Authoritative {
    #[deku(id = "1")]
    Owned,
    #[deku(id = "0")]
    Unowned,
}

#[derive(Clone, Copy, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum Truncation {
    #[deku(id = "0")]
    Complete,
    #[deku(id = "1")]
    Truncated,
}

#[derive(Clone, Copy, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum Recursion {
    #[deku(id = "0")]
    Disabled,
    #[deku(id = "1")]
    Enabled,
}

#[derive(Clone, Copy, PartialEq, Debug, DekuRead, DekuWrite, Default)]
pub struct Reserved(#[deku(bits = "3")] u8);

#[derive(Clone, Copy, PartialEq, Debug, DekuRead, DekuWrite)]
pub struct Header {
    pub id: PacketId,
    pub qr: QueryMode,
    pub op_code: OpCode,
    pub aa: Authoritative,
    pub tc: Truncation,
    pub rd: Recursion,
    pub ra: Recursion,
    pub z: Reserved,
    pub r_code: OpCode,
    #[deku(endian = "big")]
    pub qd_count: u16,
    #[deku(endian = "big")]
    pub an_count: u16,
    #[deku(endian = "big")]
    pub ar_count: u16,
    #[deku(endian = "big")]
    pub ns_count: u16,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            id: PacketId(0),
            qr: QueryMode::Query,
            op_code: OpCode::default(),
            aa: Authoritative::Unowned,
            tc: Truncation::Complete,
            ra: Recursion::Disabled,
            rd: Recursion::Enabled,
            z: Reserved::default(),
            r_code: OpCode::default(),
            qd_count: 0,
            an_count: 0,
            ar_count: 0,
            ns_count: 0,
        }
    }
}
