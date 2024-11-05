use std::time::Duration;

use crate::common::RistListenerPort;

struct Receiver {
    listen_port: RistListenerPort, // P
    sender_address: String,        // S
}

struct Config {
    buffer_size: Duration,
    reorder_section: Duration,
    max_number_of_retry_per_packet: u32,
}
