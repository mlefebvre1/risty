use risty_proto::risty_core::Marshal;
use risty_proto::rtp;

pub struct RtpConfig {
    // RTP Configr
    // pub rtp_source_port: u16,
    pub rtp_pt: u8,
    pub peer_sockaddr: String,
    // pub peer_address: IpAddr,
    // pub rtp_peer_port: RistListenerPort,
}

pub struct RtpSender {
    config: RtpConfig,
    header: rtp::Header,
}

pub struct RtpTransmit {
    // packet: Vec<u8>,
    pub remote_sockaddr: String,
    pub buf_size: usize,
}

impl RtpSender {
    pub fn new(config: RtpConfig, now: u32) -> Self {
        let header = rtp::Header::new(config.rtp_pt, now, false, None);
        Self { config, header }
    }

    pub fn poll_rtp_transmit(
        &mut self,
        payload: &[u8],
        now: u32,
        marker: bool,
        send_buf: &mut [u8],
    ) -> RtpTransmit {
        let packet = rtp::Packet::new(&self.header, payload);

        let n = packet.marshal(send_buf).unwrap();
        self.header.update(now, marker);

        RtpTransmit {
            remote_sockaddr: self.config.peer_sockaddr.clone(),
            buf_size: n,
        }
    }
}
