use super::header::{Header, PacketType, VERSION};

const GENERIC_NACK_CODE: u8 = 1;

/// Bitmask-based retransmissions shall be requested using the Generic NACK Message.
pub struct GenericNack {
    pub header: Header,

    /// The synchronization source identifier of the media source that this feedback request is related to.
    /// As indicated later in this document, the LSB of the SSRC is used to differentiate between original
    /// packets and retransmitted packets. The RIST receiver may use either value in the request packet.
    pub ssrc_media_src: u32,
}

impl GenericNack {
    pub fn new(ssrc_packet_sender: u32, ssrc_media_src: u32) -> Self {
        Self {
            header: Header {
                version: VERSION,
                padding: false,
                packet_specific: GENERIC_NACK_CODE,
                packet_type: PacketType::Feedback as u8,
                length: 0,
                ssrc: ssrc_packet_sender,
            },
            ssrc_media_src,
        }
    }
}

/// Feedback Control Information (FCI): This field contains one or more instances of the
/// 32-bit Generic NACK message shown below. Each FCI can request up to 17 lost packets.
/// A Generic NACK message may contain multiple FCI fields.
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

pub struct RangeBasedNACK {}
