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
    Unowned,
    Owned,
}

#[derive(Clone, Copy, Debug, PartialEq, Specifier)]
pub enum Truncation {
    Complete,
    Truncated,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursion_from() {
        let e = Recursion::from_bytes(1u8).unwrap();
        let d = Recursion::from_bytes(0u8).unwrap();
        assert_eq!(e, Recursion::Enabled);
        assert_eq!(d, Recursion::Disabled);
    }

    #[test]
    fn test_truncation_from() {
        let e = Truncation::from_bytes(1u8).unwrap();
        let d = Truncation::from_bytes(0u8).unwrap();
        assert_eq!(e, Truncation::Truncated);
        assert_eq!(d, Truncation::Complete);
    }

    #[test]
    fn test_authoritative_from() {
        let e = Authoritative::from_bytes(1u8).unwrap();
        let d = Authoritative::from_bytes(0u8).unwrap();
        assert_eq!(e, Authoritative::Owned);
        assert_eq!(d, Authoritative::Unowned);
    }

    #[test]
    fn test_mode_from() {
        let e = QueryMode::from_bytes(1u8).unwrap();
        let d = QueryMode::from_bytes(0u8).unwrap();
        assert_eq!(e, QueryMode::Response);
        assert_eq!(d, QueryMode::Query);
    }
}
