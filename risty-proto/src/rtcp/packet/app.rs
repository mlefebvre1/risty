use super::header::{Header, PacketType, VERSION};

const RIST_NAME: u32 = 0x52495354;
const GENERIC_NACK_CODE: u8 = 1;

/// This field identifies the type of the message
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

    /// The synchronization source identifier of the media source that this feedback request is related to.
    /// The LSB of the SSRC is used to differentiate between original packets and retransmitted packets.
    /// The RIST receiver may use either value in the request packet.
    /// * SSRC LSB=0: Original Packet
    /// * SSRC LSB=1: Retransmission Packet
    ssrc: u32,

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
    pub fn new_request(ssrc: u32, padding_size: u32) -> Self {
        Self {
            header: Header {
                version: VERSION.into(),
                padding: false,
                packet_specific: (Subtype::EchoRequest as u8).into(),
                packet_type: PacketType::App as u8,
                length: 5 + padding_size as u16,
            },
            ssrc,
            name: RIST_NAME,
            timestamp: 0x55555555AAAAAAAA, // The originator of this message (Subtype = 2) shall fill in an arbitrary value in this field
            processing_delay: 0, // In RTT Echo Request messages (Subtype = 2), the message sender shall fill this field with zeros
            padding_size,
        }
    }
}

/// Bitmask-based retransmissions shall be requested using the Generic NACK Message.
pub struct GenericNack<'a> {
    pub header: Header,

    /// The synchronization source identifier for the originator of this packet. This field
    /// shall be ignored by the RIST sender.
    pub ssrc_packet_sender: u32,

    /// The synchronization source identifier of the media source that this feedback request is related to.
    /// As indicated later in this document, the LSB of the SSRC is used to differentiate between original
    /// packets and retransmitted packets. The RIST receiver may use either value in the request packet.
    pub ssrc_media_src: u32,

    /// A Generic NACK message may contain multiple FCI fields.
    pub fcis: &'a [Fci],
}

impl<'a> GenericNack<'a> {
    pub fn new(ssrc_media_src: u32, fcis: &'a [Fci]) -> Self {
        Self {
            header: Header {
                version: VERSION.into(),
                padding: false,
                packet_specific: GENERIC_NACK_CODE.into(),
                packet_type: PacketType::Feedback as u8,
                length: 0, //TODO
            },
            ssrc_packet_sender: 0,
            ssrc_media_src,
            fcis,
        }
    }
}

/// Feedback Control Information (FCI): This field contains one or more instances of the
/// 32-bit Generic NACK message. Each FCI can request up to 17 lost packets.
pub struct Fci {
    /// The PID field is used to specify a lost packet. The PID field refers to the RTP sequence number of the lost packet.
    pub pid: u16,

    /// bitmask of following lost packets:
    /// The BLP allows for reporting losses of any of the 16 RTP packets immediately following the RTP packet indicated
    /// by the PID. Denoting the BLP's least significant bit as bit 1, and its most significant bit as bit 16, then bit
    /// i of the bit mask is set to 1 if the receiver has not received RTP packet number (PID+i) (modulo 2^16) and
    /// indicates this packet is lost; bit i is set to 0 otherwise. Note that the sender must not assume that a receiver
    /// has received a packet because its bit mask was set to 0. For example, the least significant bit of the BLP would
    /// be set to 1 if the packet corresponding to the PID and the following packet have been lost. However, the sender
    /// cannot infer that packets PID+2 through PID+16 have been received simply because bits 2 through 15 of the BLP are 0;
    /// all the sender knows is that the receiver has not reported them as lost at this time.
    pub blp: u16,
}

pub struct RangeBasedNACK<'a> {
    header: Header,

    /// The synchronization source identifier of the media source that this feedback request is related to.
    /// The LSB of the SSRC is used to differentiate between original packets and retransmitted packets.
    /// The RIST receiver may use either value in the request packet.
    /// * SSRC LSB=0: Original Packet
    /// * SSRC LSB=1: Retransmission Packet
    pub ssrc: u32,

    /// This field identifies the applications
    pub name: u32,

    pub packet_ranges: &'a [PacketRangeRequest],
}

impl<'a> RangeBasedNACK<'a> {
    pub fn new(ssrc: u32, packet_ranges: &'a [PacketRangeRequest]) -> Self {
        Self {
            header: Header {
                version: VERSION.into(),
                padding: false,
                packet_specific: GENERIC_NACK_CODE.into(),
                packet_type: PacketType::Feedback as u8,
                length: 0, //TODO
            },
            ssrc,
            name: RIST_NAME,
            packet_ranges,
        }
    }
}

/// Packet Range Requests: these are 32- bit fields, each requesting one packet range.
pub struct PacketRangeRequest {
    /// RTP sequence number of the first packet dropped in the block
    seq_start: u16,

    /// Number consecutive packets being requested after the packet identified by the
    /// missing packet sequence start. For example, the Missing Packet Sequence Start is
    /// N and the Number of Additional Missing Packets is A, this indicates that packets
    /// from N to N+A inclusive have been lost. If A is zero, then only one packet (with
    /// sequence number N) is being requested.
    nb_consecutive: u16,
}
