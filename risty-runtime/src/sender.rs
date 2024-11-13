use std::net::IpAddr;

use crate::rtcp_sender::{RtcpConfig, RtcpSender};
use crate::rtp_sender::{RtpConfig, RtpSender};

struct SenderConfig {
    rtp_config: RtpConfig,
    rtcp_config: RtcpConfig,
}

pub struct Sender {
    rtp_sender: RtpSender,
    rtcp_sender: RtcpSender,
}

impl Sender {
    pub fn new(config: SenderConfig) -> Self {
        Self {
            rtp_sender: RtpSender::new(config.rtp_config),
            rtcp_sender: RtcpSender::new(config.rtcp_config),
        }
    }
}
