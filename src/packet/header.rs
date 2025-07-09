use binrw::{BinRead, BinWrite};
use modular_bitfield::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Specifier)]
pub enum QueryMode {
    Query,
    Response,
}

#[bitfield(filled = false)]
#[derive(Clone, Copy, Debug, PartialEq, Default, Specifier)]
pub struct OpCode(B4);

#[derive(Clone, Copy, Debug, PartialEq, Specifier)]
pub enum Authoritative {
    Owned,
    Unowned,
}

#[derive(Clone, Copy, Debug, PartialEq, Specifier)]
pub enum Truncation {
    Complete,
    Truncated, // 1
}

#[derive(Clone, Copy, Debug, PartialEq, Specifier)]
pub enum Recursion {
    Disabled,
    Enabled,
}

#[bitfield(filled = false)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Specifier)]
pub struct Reserved(B3);

#[bitfield]
#[derive(Clone, Copy, PartialEq, Debug, Default, BinRead, BinWrite)]
#[br(map = Self::from_bytes)]
pub struct Flags {
    pub rd: Recursion,
    pub tc: Truncation,
    pub aa: Authoritative,
    pub op_code: OpCode,
    pub qr: QueryMode,

    pub r_code: OpCode,
    pub z: Reserved,
    pub ra: Recursion,
}

#[derive(Clone, Copy, PartialEq, Debug, Default, BinRead, BinWrite)]
#[brw(big)]
pub struct Header {
    pub id: u16,
    pub flags: Flags,
    pub qd_count: u16,
    pub an_count: u16,
    pub ar_count: u16,
    pub ns_count: u16,
}
