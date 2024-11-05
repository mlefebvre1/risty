use std::time::Duration;

use num::Integer;

struct Config {
    buffer_size: Duration,
    reorder_section: Duration,
    max_number_of_retry_per_packet: u32,
}

pub struct RistListenerPort(u16);
impl RistListenerPort {
    pub fn new(port: u16) -> Result<Self, String> {
        if port.is_even() && (2..=65534).contains(&port) {
            Ok(Self(port))
        } else {
            Err("TODO make an error enum!".to_string())
        }
    }
}
