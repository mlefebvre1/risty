pub(crate) const VERSION: u8 = 2;

pub enum PacketType {
    SenderReport = 200,
    ReceiverReport = 201,
    Sdes = 202,
    App = 204,
    Feedback = 205,
}

pub struct Header {
    /// Identifies the version of RTP, which is the same in RTCP packets as in RTP data packets.
    /// RIST packets shall have V=2.
    pub version: u8,

    /// Indicates whether or not there is padding at the end of the packet.
    /// RIST packets shall have P=0.
    pub padding: bool,

    /// This field has a specific meaning for each different kind of rtcp packets
    /// * SR and RR: reception report count -> The number of reception report blocks contained in this packet.
    /// * SDES: source count -> The number of chunks contained in this packet.
    /// * RTT Echo: Subtype -> This field identifies the type of the message.
    /// * NACK: Feedback message type -> This field identifies the type of the FB message and is interpreted relative
    ///                                to the type (transport layer, payload-specific, or application layer feedback).
    pub packet_specific: u8,

    /// Identify the RTCP packet.
    pub packet_type: u8,

    /// The length of this RTCP packet in 32-bit words minus one, including the header and any padding.
    pub length: u16,
}
