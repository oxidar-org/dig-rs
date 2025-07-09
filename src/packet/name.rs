use anyhow::Result;
use binrw::{BinRead, BinResult, BinWrite};
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct Name(String);

const STR_REF_MSB: u8 = 0b11000000;
const MAX_LENGTH: usize = 255;

impl Name {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Name {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        anyhow::ensure!(value.len() <= MAX_LENGTH);
        Ok(Name(value.to_string()))
    }
}

impl BinRead for Name {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut labels: Vec<String> = vec![];
        let is_ref = |b| (b & STR_REF_MSB) != 0;

        loop {
            let offset = u8::read_options(reader, endian, ())?;

            match offset {
                0 => break,
                offset if is_ref(offset) => {
                    let head = offset & !STR_REF_MSB;
                    let tail = u8::read_options(reader, endian, ())?;
                    let offset: u16 = (head as u16) << 8 | tail as u16;

                    let reader_pos = reader.stream_position()?;
                    reader.seek(SeekFrom::Start(offset as u64))?;

                    let name = Self::read_options(reader, endian, args)?;
                    labels.push(name.0);

                    reader.seek(SeekFrom::Start(reader_pos))?;
                    break;
                }
                offset => {
                    let mut buffer = vec![0; offset as usize];
                    reader.read_exact(&mut buffer)?;
                    let part = std::str::from_utf8(&buffer[..offset as usize]).unwrap();
                    labels.push(part.to_string());
                }
            }
        }

        Ok(Self(labels.join(".")))
    }
}

impl BinWrite for Name {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let labels = self.0.split('.').collect::<Vec<_>>();
        for label in labels {
            writer.write_all(&[label.len() as u8])?;
            writer.write_all(label.as_bytes())?;
        }
        writer.write_all(&[0])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_name_from_str() {
        let name = Name::try_from("example.com").unwrap();
        assert_eq!(name.as_str(), "example.com");
    }

    #[test]
    fn test_name_from_str_with_invalid_length() {
        let result = Name::try_from("a".repeat(MAX_LENGTH + 1).as_str());
        assert!(result.is_err());
    }

    #[test]
    fn test_name_from_bytes() {
        let mut bytes = Cursor::new(b"\x03www\x07example\x03com\x00");
        let name = Name::read_be(&mut bytes).unwrap();
        assert_eq!(name.as_str(), "www.example.com");
    }
}
