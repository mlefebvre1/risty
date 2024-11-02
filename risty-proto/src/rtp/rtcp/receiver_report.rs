use super::{header::Header, header::PacketType, header::VERSION, report_block::ReportBlock};
use packed_struct::prelude::*;
use risty_core::{Marshal, MarshalError};

const EMPTY_RR_LENGTH: u16 = 1;
const RR_LENGTH: u16 = 7;

/// Receiver report, for reception statistics from participants that are not active senders and
/// in combination with SR for active senders reporting on more than 31 sources
/// * Note: Typically RR can include more than 1 report block, but in RIST this is fixed to
///     0 for empty RR or 1 for RR.
struct ReceiverReport {
    pub header: Header,

    /// The synchronization source identifier for the originator of this SR packet.
    pub ssrc_sender: u32,

    pub report_block: Vec<ReportBlock>,
}

impl ReceiverReport {
    pub fn new_with_report_block(ssrc_sender: u32, report_block: ReportBlock) -> Self {
        Self {
            header: Header {
                version: VERSION.into(),
                padding: false,
                packet_specific: 1.into(),
                packet_type: PacketType::ReceiverReport as u8,
                length: RR_LENGTH,
            },
            ssrc_sender,
            report_block: vec![report_block],
        }
    }

    pub fn new_empty(ssrc_sender: u32) -> Self {
        Self {
            header: Header {
                version: VERSION.into(),
                padding: false,
                packet_specific: 0.into(),
                packet_type: PacketType::ReceiverReport as u8,
                length: EMPTY_RR_LENGTH,
            },
            ssrc_sender,
            report_block: vec![],
        }
    }
}

impl Marshal for ReceiverReport {
    fn marshal(&self, buf: &mut [u8]) -> Result<usize, MarshalError> {
        self.header.pack_to_slice(&mut buf[0..=3])?;

        buf[4..=7].copy_from_slice(&self.ssrc_sender.to_msb_bytes());

        for block in &self.report_block {
            block.pack_to_slice(&mut buf[8..=31])?;
        }

        Ok(self.marshal_size())
    }

    fn marshal_size(&self) -> usize {
        8 + 24 * self.report_block.len()
    }
}
