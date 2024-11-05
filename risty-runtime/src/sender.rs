use crate::common::RistListenerPort;

pub struct Sender {
    receiver_address: String,
    receiver_port: RistListenerPort, // P
    rtcp_listener_port: u16,         // R
}
