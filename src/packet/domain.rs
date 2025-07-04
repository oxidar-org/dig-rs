use super::Name;
use deku::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u16", endian = "big")]
pub enum Record {
    #[deku(id = "1")]
    AA,
}

#[derive(Clone, Copy, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u16", endian = "big")]
pub enum Class {
    #[deku(id = "1")]
    IN,
}

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
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

    #[test]
    fn test_domain_new_aa() {
        let domain = Domain::new_aa("example.com".try_into().unwrap());
        assert_eq!(domain.name(), "example.com");
        assert_eq!(domain.record, Record::AA);
        assert_eq!(domain.class, Class::IN);
    }

    #[test]
    fn test_domain_to_bytes() {
        let domain = Domain::new_aa("google.com".try_into().unwrap());
        let bytes = domain.to_bytes().unwrap();

        let chunk: &[u8] = &[
            0x6, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0, 0, 1, 0, 1,
        ];
        assert_eq!(&bytes, chunk);
    }
}
