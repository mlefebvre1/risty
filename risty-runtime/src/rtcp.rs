use std::time::Duration;

struct Config {
    interval: Duration, // 100ms or less
    enable_rtt_echoes: bool,
}
