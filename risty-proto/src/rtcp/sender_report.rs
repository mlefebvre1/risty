use super::{header::Header, header::PacketType, header::VERSION};

const SR_LENGTH: u16 = 6;

pub struct SenderReport {
    header: Header,

    /// Indicates the wallclock time when this report was sent. The most significant 32 bits on this field
    /// indicate the number of seconds since 0h UTC on January 1900, and the least significant 32 bits
    /// indicate the fraction of the second.On a system that has no notion of wallclock time but does
    /// have some system-specific clock such as "System uptime", a sender may use that clock as a reference
    /// to calculate relative NTP timestamps. A sender that has no notion of wallclock or elapsed time may
    /// set the NTP timestamp to zero.
    ntp_ts: u64,

    /// Corresponds to the same time as the NTP timestamp (above), but in the same units and with the same random
    /// offset as the RTP timestamps in data packets. Note that in most cases this timestamp will not be equal
    /// to the RTP timestamp in any adjacent data packet. Rather, it shall be calculated from the corresponding
    /// NTP timestamp using the relationship between the RTP timestamp counter and real time as maintained by
    /// periodically checking the wallclock time at a sampling instant.
    rtp_ts: u32,

    /// The total number of RTP data packets transmitted by the sender since starting transmission up until the time
    /// this SR packet was generated. The count should be reset if the sender changes its SSRC identifier.
    sender_packet_count: u32,

    /// The total number of payload octets (i.e., not including header or padding) transmitted in RTP data packets
    /// by the sender since starting transmission up until the time this SR packet was generated. The count should
    /// be reset if the sender changes its SSRC identifier.
    sender_octet_count: u32,
}

impl Default for SenderReport {
    fn default() -> Self {
        Self {
            header: Header {
                version: VERSION,
                padding: false,
                packet_specific: 0,
                packet_type: PacketType::SenderReport as u8,
                length: SR_LENGTH,
                ssrc: 0,
            },
            ntp_ts: 0,
            rtp_ts: 0,
            sender_packet_count: 0,
            sender_octet_count: 0,
        }
    }
}

impl SenderReport {
    pub fn new(ssrc: u32) -> Self {
        let mut sr = SenderReport::default();
        sr.header.ssrc = ssrc;
        sr
    }

    pub fn update_ntp_ts(&mut self, ntp_ts: u64) {
        self.ntp_ts = ntp_ts;
    }

    pub fn update_rtp_ts(&mut self, rtp_ts: u32) {
        self.rtp_ts = rtp_ts;
    }

    pub fn update_sender_packet_count(&mut self, count: u32) {
        self.sender_packet_count = count;
    }

    pub fn update_sender_octet_count(&mut self, count: u32) {
        self.sender_octet_count = count;
    }
}
