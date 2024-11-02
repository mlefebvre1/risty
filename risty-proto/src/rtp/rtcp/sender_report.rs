use super::{header::Header, header::PacketType, header::VERSION};
use packed_struct::prelude::*;
use risty_core::Marshal;

const SR_LENGTH: u16 = 6;

/// Sender report, for transmission and reception statistics from participants that are active senders.
/// * Note: Typically SenderReport can include 1 or many Report Blocks, but in RIST this is not used, so this
///     is ommited in the struct.
#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "msb0")]
pub struct SenderReport {
    #[packed_field(bytes = "0..=3", endian = "msb")]
    header: Header,

    /// The synchronization source identifier for the originator of this SR packet.
    #[packed_field(bytes = "4..=7", endian = "msb")]
    ssrc_sender: u32,

    #[packed_field(bytes = "8..=27", endian = "msb")]
    sender_info: SenderInfo,
}

impl SenderReport {
    pub fn new(sender_ssrc: u32) -> Self {
        Self {
            header: Header {
                version: VERSION.into(),
                padding: false,
                packet_specific: 0.into(),
                packet_type: PacketType::SenderReport as u8,
                length: SR_LENGTH,
            },
            ssrc_sender: sender_ssrc,
            sender_info: SenderInfo {
                ntp_ts: 0,
                rtp_ts: 0,
                sender_packet_count: 0,
                sender_octet_count: 0,
            },
        }
    }
}

impl Marshal for SenderReport {
    fn marshal(&self, buf: &mut [u8]) -> Result<usize, risty_core::MarshalError> {
        self.pack_to_slice(&mut buf[0..=27])?;
        Ok(self.marshal_size())
    }
    fn marshal_size(&self) -> usize {
        28
    }
}

/// It summarizes the data transmissions from this sender.
#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "msb0")]
pub struct SenderInfo {
    /// Indicates the wallclock time when this report was sent. The most significant 32 bits on this field
    /// indicate the number of seconds since 0h UTC on January 1900, and the least significant 32 bits
    /// indicate the fraction of the second.On a system that has no notion of wallclock time but does
    /// have some system-specific clock such as "System uptime", a sender may use that clock as a reference
    /// to calculate relative NTP timestamps. A sender that has no notion of wallclock or elapsed time may
    /// set the NTP timestamp to zero.
    #[packed_field(bytes = "0..=7", endian = "msb")]
    pub ntp_ts: u64,

    /// Corresponds to the same time as the NTP timestamp (above), but in the same units and with the same random
    /// offset as the RTP timestamps in data packets. Note that in most cases this timestamp will not be equal
    /// to the RTP timestamp in any adjacent data packet. Rather, it shall be calculated from the corresponding
    /// NTP timestamp using the relationship between the RTP timestamp counter and real time as maintained by
    /// periodically checking the wallclock time at a sampling instant.
    #[packed_field(bytes = "8..=11", endian = "msb")]
    pub rtp_ts: u32,

    /// The total number of RTP data packets transmitted by the sender since starting transmission up until the time
    /// this SR packet was generated. The count should be reset if the sender changes its SSRC identifier.
    #[packed_field(bytes = "12..=15", endian = "msb")]
    pub sender_packet_count: u32,

    /// The total number of payload octets (i.e., not including header or padding) transmitted in RTP data packets
    /// by the sender since starting transmission up until the time this SR packet was generated. The count should
    /// be reset if the sender changes its SSRC identifier.
    #[packed_field(bytes = "16..=19", endian = "msb")]
    pub sender_octet_count: u32,
}
