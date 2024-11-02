use super::Subtype;
use crate::rtp::rtcp::header::{Header, PacketType, VERSION};

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
                packet_specific: (Subtype::GenericNack as u8).into(),
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
