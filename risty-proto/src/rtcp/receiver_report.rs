use super::{header::Header, header::PacketType, header::VERSION};

const EMPTY_RR_LENGTH: u16 = 1;
const RR_LENGTH: u16 = 7;

struct EmptyReceiverReport {
    header: Header,
}

impl Default for EmptyReceiverReport {
    fn default() -> Self {
        Self {
            header: Header {
                version: VERSION,
                padding: false,
                packet_specific: 0,
                packet_type: PacketType::ReceiverReport as u8,
                length: EMPTY_RR_LENGTH,
                ssrc: 0,
            },
        }
    }
}

impl EmptyReceiverReport {
    pub fn new(ssrc: u32) -> Self {
        let mut empty_rr = EmptyReceiverReport::default();
        empty_rr.header.ssrc = ssrc;
        empty_rr
    }
}

struct ReceiverReport {
    header: Header,

    /// The fraction of RTP data packets from source SSRC_n lost since the
    /// previous SR or RR packet was sent, expressed as a fixed point
    /// number with the binary point at the left edge of the field.
    fraction_lost: u8,

    ///  The total number of RTP data packets from source SSRC_n that have been lost since the beginning of reception.
    cumm_packets_lost: u32,

    /// The low 16 bits contain the highest sequence number received in an RTP data packet from source SSRC_n,
    ///  and the most significant 16 bits extend that sequence number with the corresponding count of sequence number cycles.
    highest_extended_seq_num_received: u32,

    /// An estimate of the statistical variance of the RTP data packet interarrival time, measured in timestamp units and
    ///  expressed as an unsigned integer.
    interarrival_jitter: u32,

    ///  The middle 32 bits out of 64 in the NTP timestamp received as part of the most recent RTCP sender report (SR)
    ///  packet from source SSRC_n. If no SR has been received yet, the field is set to zero.
    last_sr_timestamp: u32,

    /// The delay, expressed in units of 1/65536 seconds, between receiving the last SR packet from source SSRC_n
    /// and sending this reception report block.
    delay_since_last_sr: u32,
}

impl Default for ReceiverReport {
    fn default() -> Self {
        Self {
            header: Header {
                version: VERSION,
                padding: false,
                packet_specific: 1,
                packet_type: PacketType::ReceiverReport as u8,
                length: RR_LENGTH,
                ssrc: 0,
            },
            fraction_lost: 0,
            cumm_packets_lost: 0,
            highest_extended_seq_num_received: 0,
            interarrival_jitter: 0,
            last_sr_timestamp: 0,
            delay_since_last_sr: 0,
        }
    }
}

impl ReceiverReport {
    pub fn new(ssrc: u32) -> Self {
        let mut rr = ReceiverReport::default();
        rr.header.ssrc = ssrc;
        rr
    }
}
