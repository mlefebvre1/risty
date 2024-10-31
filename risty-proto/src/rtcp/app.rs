use super::header::{Header, PacketType, VERSION};

const RIST_NAME: u32 = 0x52495354;

pub enum Subtype {
    RangeBasedNACK = 0,
    EchoRequest = 2,
    EchoResponse = 3,
}

/// The purpose of the RTCP RTT Echo Request/Response packets is to allow RIST endpoints to measure
/// the Round Trip Time (RTT) to the remote endpoint. The RTT information can be used by receivers
/// to optimize their retransmission requests
pub struct RttEcho {
    header: Header,

    /// This field identifies the application
    name: u32,

    /// The originator of this message (Subtype = 2) shall fill in an arbitrary value in this field,
    /// and the recipient of the message shall echo it back in the response (Subtype = 3). In order
    /// to aid debugging, the timestamp may be in NTP format: the Timestamp most significant word may
    /// be a value in seconds, and the Timestamp least significant word may be the fractional part.
    /// There is no requirement that this be the actual NTP time or that the nodes be NTP synchronized.
    timestamp: u64,

    /// The processing time is defined as the interval between the instant the RTT Echo Request message
    /// is received and the RTT Echo Response message is transmitted.
    processing_delay: u64,

    /// The RTT Echo Request sender may want to measure the RTT for a packet of a certain size, so it may
    /// pad the packet with a number of additional bytes, with arbitrary content. The only constraints are
    /// that the number of padding bytes shall be a multiple of 4, and the resulting compound RTCP packet
    /// shall not exceed the link MTU.
    /// This field corresponds to the number of 32 bits padding.
    padding_size: u32,
}

impl RttEcho {
    /// Creates a new RTT echo request.
    /// - `padding_size` is the number of 32 bits padding you want.
    pub fn new_request(ssrc: u32, padding_size: u32) {
        let echo = Self {
            header: Header {
                version: VERSION,
                padding: false,
                packet_specific: Subtype::EchoRequest as u8,
                packet_type: PacketType::App as u8,
                length: 5 + padding_size as u16,
                ssrc: 0,
            },
            name: RIST_NAME,
            timestamp: 0x55555555AAAAAAAA, // The originator of this message (Subtype = 2) shall fill in an arbitrary value in this field
            processing_delay: 0, // In RTT Echo Request messages (Subtype = 2), the message sender shall fill this field with zeros
            padding_size,
        };
    }
}
