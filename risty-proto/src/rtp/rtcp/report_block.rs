use packed_struct::prelude::*;

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
