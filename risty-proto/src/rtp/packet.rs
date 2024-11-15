use risty_core::Marshal;

use crate::rtp::header::Header;

pub struct Packet<'a> {
    header: Header,
    payload: &'a [u8],
}

impl<'a> Marshal for Packet<'a> {
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

impl<'a> Packet<'a> {
    pub fn new(header: Header, payload: &'a [u8]) -> Self {
        Self { header, payload }
    }
}
