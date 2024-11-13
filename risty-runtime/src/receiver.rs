use std::time::Duration;

use crate::common::RistListenerPort;

struct Receiver {
    listen_port: RistListenerPort, // P
    /// RIST receivers shall listen on UDP port P+1 for RTCP packets from the sender. The source
    /// IP address of such packets is denoted by S and their source UDP port is denoted by R’.
    sender_address: String, // S
    sender_rtcp_port: u16,         // R': obtained from

    use_upnp: bool, // TODO
}

struct Config {
    buffer_size: Duration,
    reorder_section: Duration,
    max_number_of_retry_per_packet: u32,
}

struct Capabilities {
    multicast: bool,
    rtt_echo: bool,
}
