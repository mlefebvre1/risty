use super::{header::Header, header::PacketType, header::VERSION};

const SR_LENGTH: u16 = 6;

/// Sender report, for transmission and reception statistics from participants that are active senders.
/// * Note: Typically SenderReport can include 1 or many Report Blocks, but in RIST this is not used, so this
///     is ommited in the struct.
pub struct SenderReport {
    header: Header,

    /// The synchronization source identifier for the originator of this SR packet.
    ssrc_sender: u32,

    sender_info: SenderInfo,
}

impl SenderReport {
    pub fn new(sender_ssrc: u32) -> Self {
        Self {
            header: Header {
                version: VERSION,
                padding: false,
                packet_specific: 0,
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

/// It summarizes the data transmissions from this sender.
pub struct SenderInfo {
    /// Indicates the wallclock time when this report was sent. The most significant 32 bits on this field
    /// indicate the number of seconds since 0h UTC on January 1900, and the least significant 32 bits
    /// indicate the fraction of the second.On a system that has no notion of wallclock time but does
    /// have some system-specific clock such as "System uptime", a sender may use that clock as a reference
    /// to calculate relative NTP timestamps. A sender that has no notion of wallclock or elapsed time may
    /// set the NTP timestamp to zero.
    pub ntp_ts: u64,

    /// Corresponds to the same time as the NTP timestamp (above), but in the same units and with the same random
    /// offset as the RTP timestamps in data packets. Note that in most cases this timestamp will not be equal
    /// to the RTP timestamp in any adjacent data packet. Rather, it shall be calculated from the corresponding
    /// NTP timestamp using the relationship between the RTP timestamp counter and real time as maintained by
    /// periodically checking the wallclock time at a sampling instant.
    pub rtp_ts: u32,

    /// The total number of RTP data packets transmitted by the sender since starting transmission up until the time
    /// this SR packet was generated. The count should be reset if the sender changes its SSRC identifier.
    pub sender_packet_count: u32,

    /// The total number of payload octets (i.e., not including header or padding) transmitted in RTP data packets
    /// by the sender since starting transmission up until the time this SR packet was generated. The count should
    /// be reset if the sender changes its SSRC identifier.
    pub sender_octet_count: u32,
}
