use std::net::IpAddr;

use crate::common::RistListenerPort;

pub struct RtpConfig {
    // RTP Configr
    rtp_source_port: u16,
    rtp_pt: u8,
    rtp_peer_port: RistListenerPort,
    peer_address: IpAddr,
}

pub struct RtpSender {
    config: RtpConfig,
}

impl RtpSender {
    pub fn new(config: RtpConfig) -> Self {
        Self { config }
    }

    pub fn poll_rtp_transmit(&mut self) {}
}
