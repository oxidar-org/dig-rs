use anyhow::Result;
use deku::{ctx::Order, prelude::*};

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Name(
    #[deku(
        reader = "Self::read(deku::reader)",
        writer = "Self::write(deku::writer, &self.0)"
    )]
    String,
);

impl Name {
    const MAX_LENGTH: usize = 255;

    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<String, DekuError> {
        let mut labels: Vec<String> = vec![];
        let mut peek_label = true;
        while peek_label {
            let mut buffer = [0u8; 256];
            reader.read_bytes(1, &mut buffer, Order::Msb0)?;
            let length = buffer[0] as usize;
            peek_label = length != 0;
            if peek_label {
                reader.read_bytes(length, &mut buffer, Order::Msb0)?;
                let label = std::str::from_utf8(&buffer[..length])
                    .map_err(|_| DekuError::AssertionNoStr)?;
                labels.push(label.to_string());
            }
        }
        Ok(labels.join("."))
    }

    fn write<W: std::io::Write + std::io::Seek>(
        w: &mut Writer<W>,
        s: &str,
    ) -> Result<(), DekuError> {
        for l in s.split('.') {
            w.write_bytes(&[l.len() as u8])?;
            w.write_bytes(l.as_bytes())?;
        }
        w.write_bytes(&[0u8])?;
        Ok(())
    }
}

impl Name {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Name {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        anyhow::ensure!(value.len() <= Name::MAX_LENGTH);
        Ok(Name(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_from_str() {
        let name = Name::try_from("example.com").unwrap();
        assert_eq!(name.as_str(), "example.com");
    }

    #[test]
    fn test_name_from_str_with_invalid_length() {
        let result = Name::try_from("a".repeat(Name::MAX_LENGTH + 1).as_str());
        assert!(result.is_err());
    }

    #[test]
    fn test_name_from_bytes() {
        let bytes = b"\x03www\x07example\x03com\x00";
        let (_, name) = Name::from_bytes((bytes, 0)).unwrap();
        assert_eq!(name.as_str(), "www.example.com");
    }
}
