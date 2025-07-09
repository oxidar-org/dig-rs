use super::Name;
use binrw::{BinRead, BinWrite, binrw};

#[derive(Clone, Copy, Debug, PartialEq, BinRead, BinWrite)]
#[brw(big)]
pub enum Record {
    #[brw(magic(1u16))]
    AA,
}

#[derive(Clone, Copy, Debug, PartialEq, BinRead, BinWrite)]
#[brw(big)]
pub enum Class {
    #[brw(magic(1u16))]
    IN,
}

#[derive(Clone, Debug, PartialEq)]
#[binrw]
#[brw(big)]
pub struct Domain {
    name: Name,
    record: Record,
    class: Class,
}

impl Domain {
    pub fn new_aa(name: Name) -> Self {
        Self {
            name,
            record: Record::AA,
            class: Class::IN,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_domain_new_aa() {
        let domain = Domain::new_aa("example.com".try_into().unwrap());
        assert_eq!(domain.name.as_str(), "example.com");
        assert_eq!(domain.record, Record::AA);
        assert_eq!(domain.class, Class::IN);
    }

    #[test]
    fn test_domain_to_bytes() {
        let domain = Domain::new_aa("google.com".try_into().unwrap());
        let mut writer = Cursor::new(Vec::new());
        domain.write(&mut writer).unwrap();

        assert_eq!(
            writer.into_inner(),
            &[
                0x6, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0, 0, 1, 0, 1,
            ]
        );
    }
}
