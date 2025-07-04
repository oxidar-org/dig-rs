use crate::packet::Route;
use crate::packet::header::PacketId;

use super::Domain;
use super::Header;
use deku::prelude::*;

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Message {
    header: Header,
    #[deku(count = "header.qd_count")]
    questions: Vec<Domain>,
    #[deku(count = "header.an_count")]
    answers: Vec<Route>,
}

impl Message {
    pub fn query_domain(id: u16, domain: Domain) -> Self {
        let id = PacketId(id);
        let header = Header {
            id,
            qd_count: 1,
            ..Default::default()
        };
        Self {
            header,
            questions: vec![domain],
            answers: vec![],
        }
    }

    pub fn answers(&self) -> &[Route] {
        &self.answers
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
        let bytes = message.to_bytes().unwrap();
        assert_eq!(
            bytes,
            vec![
                0x86, 0x2a, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x67,
                0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01
            ]
        );
    }
}
