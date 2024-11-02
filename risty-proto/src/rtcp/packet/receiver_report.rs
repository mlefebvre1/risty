use super::{header::Header, header::PacketType, header::VERSION};
use packed_struct::prelude::*;
use risty_core::{Marshal, MarshalError};

const EMPTY_RR_LENGTH: u16 = 1;
const RR_LENGTH: u16 = 7;

/// Receiver report, for reception statistics from participants that are not active senders and
/// in combination with SR for active senders reporting on more than 31 sources
struct ReceiverReport {
    header: Header,

    /// The synchronization source identifier for the originator of this SR packet.
    ssrc_sender: u32,

    /// Typically RR can include more than 1 report block, but in RIST this is fixed to
    /// 0 for empty RR or 1 for RR.
    report_block: Option<ReportBlock>,
}

impl ReceiverReport {
    pub fn new(ssrc_sender: u32) -> Self {
        Self {
            header: Self::rr_header(),
            ssrc_sender,
            report_block: Some(ReportBlock {
                ssrc: 0,
                fraction_lost: 0,
                cumm_packets_lost: 0,
                highest_extended_seq_num_received: 0,
                interarrival_jitter: 0,
                last_sr_timestamp: 0,
                delay_since_last_sr: 0,
            }),
        }
    }

    pub fn new_with_report_block(ssrc_sender: u32, report_block: ReportBlock) -> Self {
        Self {
            header: Self::rr_header(),
            ssrc_sender,
            report_block: Some(report_block),
        }
    }

    pub fn new_empty(ssrc_sender: u32) -> Self {
        Self {
            header: Self::empty_rr_header(),
            ssrc_sender,
            report_block: None,
        }
    }

    fn rr_header() -> Header {
        Header {
            version: VERSION.into(),
            padding: false,
            packet_specific: 1.into(),
            packet_type: PacketType::ReceiverReport as u8,
            length: RR_LENGTH,
        }
    }

    fn empty_rr_header() -> Header {
        Header {
            version: VERSION.into(),
            padding: false,
            packet_specific: 0.into(),
            packet_type: PacketType::ReceiverReport as u8,
            length: EMPTY_RR_LENGTH,
        }
    }
}

impl Marshal for ReceiverReport {
    fn marshal(&self, buf: &mut [u8]) -> Result<usize, MarshalError> {
        self.header.pack_to_slice(&mut buf[0..=3])?;

        buf[4..=7].copy_from_slice(&self.ssrc_sender.to_msb_bytes());

        if let Some(block) = &self.report_block {
            block.pack_to_slice(&mut buf[8..=31])?;
            Ok(32)
        } else {
            Ok(8)
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(bit_numbering = "msb0")]
pub struct ReportBlock {
    /// The SSRC identifier of the source to which the information in this reception report block pertains.
    #[packed_field(bytes = "0..=3", endian = "msb")]
    pub ssrc: u32,

    /// The fraction of RTP data packets from source SSRC_n lost since the
    /// previous SR or RR packet was sent, expressed as a fixed point
    /// number with the binary point at the left edge of the field.
    #[packed_field(bytes = "4")]
    pub fraction_lost: u8,

    ///  The total number of RTP data packets from source SSRC_n that have been lost since the beginning of reception.
    #[packed_field(bytes = "5..=7", endian = "msb")]
    pub cumm_packets_lost: u32,

    /// The low 16 bits contain the highest sequence number received in an RTP data packet from source SSRC_n,
    ///  and the most significant 16 bits extend that sequence number with the corresponding count of sequence number cycles.
    #[packed_field(bytes = "8..=11", endian = "msb")]
    pub highest_extended_seq_num_received: u32,

    /// An estimate of the statistical variance of the RTP data packet interarrival time, measured in timestamp units and
    ///  expressed as an unsigned integer.
    #[packed_field(bytes = "12..=15", endian = "msb")]
    pub interarrival_jitter: u32,

    ///  The middle 32 bits out of 64 in the NTP timestamp received as part of the most recent RTCP sender report (SR)
    ///  packet from source SSRC_n. If no SR has been received yet, the field is set to zero.
    #[packed_field(bytes = "16..=19", endian = "msb")]
    pub last_sr_timestamp: u32,

    /// The delay, expressed in units of 1/65536 seconds, between receiving the last SR packet from source SSRC_n
    /// and sending this reception report block.
    #[packed_field(bytes = "20..=23", endian = "msb")]
    pub delay_since_last_sr: u32,
}
