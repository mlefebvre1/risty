use std::time::Instant;

use super::Subtype;
use crate::rtp::rtcp::header::{Header, PacketType, VERSION};

use packed_struct::prelude::*;
use risty_core::Marshal;

const RIST_NAME: u32 = 0x52495354;

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
    pub fn new_request(ssrc: u32, padding_size: u32, now: Instant) -> Self {
        Self {
            header: Header {
                version: VERSION.into(),
                padding: false,
                packet_specific: (Subtype::EchoRequest as u8).into(),
                packet_type: PacketType::App as u8,
                length: Self::calculate_length(padding_size),
            },
            ssrc,
            name: RIST_NAME,
            timestamp: 0x55555555AAAAAAAA, // The originator of this message (Subtype = 2) shall fill in an arbitrary value in this field
            processing_delay: 0, // In RTT Echo Request messages (Subtype = 2), the message sender shall fill this field with zeros
            padding_size,
        }
    }

    pub fn new_response(
        ssrc: u32,
        timestamp: u64,
        processing_delay: u64,
        padding_size: u32,
    ) -> Self {
        Self {
            header: Header {
                version: VERSION.into(),
                padding: false,
                packet_specific: (Subtype::EchoResponse as u8).into(),
                packet_type: PacketType::App as u8,
                length: Self::calculate_length(padding_size),
            },
            ssrc,
            name: RIST_NAME,
            timestamp,
            processing_delay,
            padding_size,
        }
    }

    fn calculate_length(padding_size: u32) -> u16 {
        5 + padding_size as u16
    }
}

impl Marshal for RttEcho {
    fn marshal(&self, buf: &mut [u8]) -> Result<usize, risty_core::MarshalError> {
        self.header.pack_to_slice(&mut buf[0..4])?;
        buf[4..8].copy_from_slice(&self.ssrc.to_be_bytes());
        buf[8..12].copy_from_slice(&self.name.to_be_bytes());
        buf[12..20].copy_from_slice(&self.timestamp.to_be_bytes());
        buf[20..28].copy_from_slice(&self.processing_delay.to_be_bytes());

        Ok(self.marshal_size())
    }

    fn marshal_size(&self) -> usize {
        28 + 4 * self.padding_size as usize
    }
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
                packet_specific: (Subtype::RangeBasedNACK as u8).into(),
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
