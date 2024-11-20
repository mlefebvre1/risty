use std::time::Duration;

use risty_proto::risty_core::{Marshal, RtpClock};
use risty_proto::rtp;

pub struct RtpConfig {
    // RTP Configr
    // pub rtp_source_port: u16,
    pub rtp_pt: u8,
    pub peer_sockaddr: String,
    pub rtp_clock_frequency: u32,
    // pub peer_address: IpAddr,
    // pub rtp_peer_port: RistListenerPort,
}

pub struct RtpSender {
    config: RtpConfig,
    header: rtp::Header,
    clock: RtpClock,
}

pub struct RtpTransmit {
    // packet: Vec<u8>,
    pub remote_sockaddr: String,
    pub buf_size: usize,
}

impl RtpSender {
    pub fn new(config: RtpConfig, now: Duration) -> Self {
        let clock = RtpClock::new(config.rtp_clock_frequency);
        let header = rtp::Header::new(
            config.rtp_pt,
            clock.timestamp_from_duration(now),
            false,
            None,
        );
        Self {
            config,
            header,
            clock,
        }
    }

    pub fn poll_rtp_transmit(
        &mut self,
        payload: &[u8],
        now: Duration,
        marker: bool,
        send_buf: &mut [u8],
    ) -> RtpTransmit {
        let packet = rtp::Packet::new(&self.header, payload);

        let n = packet.marshal(send_buf).unwrap();
        self.header
            .update(self.clock.timestamp_from_duration(now), marker);

        RtpTransmit {
            remote_sockaddr: self.config.peer_sockaddr.clone(),
            buf_size: n,
        }
    }
}
