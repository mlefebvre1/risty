use crate::rtp_sender::{RtpConfig, RtpSender};

struct SenderConfig {
    rtp_config: RtpConfig,
}

pub struct Sender {
    rtp_sender: RtpSender,
}

// impl Sender {
//     pub fn new(config: SenderConfig) -> Self {
//         Self {
//             rtp_sender: RtpSender::new(config.rtp_config),
//         }
//     }
// }
