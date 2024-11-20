use num::Integer;

// struct Config {
//     buffer_size: Duration,
//     max_number_of_retry_per_packet: u32,
// }

/// RIST senders shall transmit the RTP media packets to the configured IP address of the RIST
/// receiver and a user-selected UDP destination port P, where P is an even number between 2
/// and 65534.
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
