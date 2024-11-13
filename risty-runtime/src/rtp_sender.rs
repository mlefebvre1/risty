use std::net::IpAddr;

use crate::common::RistListenerPort;

pub struct RtpConfig {
    // RTP Config
    rtp_source_port: u16, // M
    rtp_pt: u8,
    rtp_peer_port: RistListenerPort,
    peer_address: IpAddr,

    // RTCP Config.
    /// The sender may choose any arbitrary source port M for the RTP flow
    /// RIST senders may offer the user the ability to manually configure source ports M
    rtcp_listener_port: u16,

    // Buffer Config.
    buffer_size: u64,
}

pub struct RtpSender {
    config: RtpConfig,
}

impl RtpSender {
    pub fn new(config: RtpConfig) -> Self {
        Self { config }
    }

    /// RIST senders shall periodically transmit the compound RTCP packets specified in section
    /// 5.2.1 to the configured IP address of the RIST receiver and UDP port P+1
    pub fn rtcp_receiver_port(&self) -> u16 {
        self.rtcp_listener_port + 1
    }

    /// this function shall be called when receiving a packet on the rtcp socket
    pub fn handle_rtcp_input(&mut self, packet: &[u8]) {}

    /// this function shall be called to send a rtcp packet to the receiver
    pub fn poll_rtcp_transmit(&mut self) {}

    pub fn poll_rtp_transmit(&mut self) {}
}
