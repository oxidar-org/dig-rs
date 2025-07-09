use crate::packet::header::Recursion;

use super::{Domain, Header, Route};
use anyhow::Result;
use binrw::prelude::*;
use std::io::Cursor;

#[derive(Clone, Debug, PartialEq, BinRead, BinWrite)]
pub struct Message {
    header: Header,
    #[br(args { count: header.qd_count.into() })]
    questions: Vec<Domain>,
    #[br(args { count: header.an_count.into() })]
    answers: Vec<Route>,
}

impl Message {
    pub fn query_domain(id: u16, domain: Domain) -> Self {
        let mut header = Header {
            id,
            qd_count: 1,
            ..Default::default()
        };
        header.flags.set_rd(Recursion::Enabled);
        Self {
            header,
            questions: vec![domain],
            answers: vec![],
        }
    }

    pub fn answers(&self) -> &[Route] {
        &self.answers
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_be(&mut writer).unwrap();
        writer.into_inner()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut reader = Cursor::new(bytes);
        let m = Self::read_be(&mut reader)?;
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_domain() {
        let domain = Domain::new_aa("example.com".try_into().unwrap());
        let message = Message::query_domain(34346, domain);
        assert_eq!(message.questions.len(), 1);
        assert_eq!(message.questions[0].name(), "example.com");
    }

    #[test]
    fn test_query_domain_bytes() {
        let domain = Domain::new_aa("google.com".try_into().unwrap());
        let message = Message::query_domain(34346, domain);
        let mut writer = Cursor::new(Vec::new());
        message.write_be(&mut writer).unwrap();

        assert_eq!(
            writer.into_inner(),
            vec![
                0x86, 0x2a, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x67,
                0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01
            ]
        );
    }

    #[test]
    fn test_parse_response() {
        let data = std::fs::read("packet-resp.bin").unwrap();
        assert!(!data.is_empty());

        let msg = Message::from_bytes(&data).unwrap();
        assert_eq!(msg.questions.len(), 1);
        assert_eq!(msg.answers().len(), 4);
    }
}
