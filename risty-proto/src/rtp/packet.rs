use risty_core::Marshal;

use crate::rtp::header::Header;

pub struct Packet {
    header: Header,
    payload: Vec<u8>,
}

impl Marshal for Packet {
    fn marshal(&self, buf: &mut [u8]) -> Result<usize, risty_core::MarshalError> {
        let header_size = self.header.marshal_size();
        let _ = self.header.marshal(&mut buf[0..header_size])?;

        let payload_size = self.payload.len();
        let packet_size = self.marshal_size();
        buf[header_size..packet_size].copy_from_slice(&self.payload[0..payload_size]);

        Ok(packet_size)
    }

    fn marshal_size(&self) -> usize {
        self.header.marshal_size() + self.payload.len()
    }
}

impl Packet {
    pub fn new(header: Header, payload: Vec<u8>) -> Self {
        Self { header, payload }
    }
}
