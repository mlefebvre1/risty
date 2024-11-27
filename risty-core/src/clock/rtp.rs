use std::time::{Duration, Instant, SystemTime, SystemTimeError, UNIX_EPOCH};

pub struct RtpClock {
    freq: u32,
}

impl RtpClock {
    pub fn new(freq: u32) -> Self {
        Self { freq }
    }

    /// We are calculating the following formula:
    /// rtp_time = (duration(s) * rtp_clock_frequency + duration(fract) * rtp_clock_frequency) % 2**32
    pub fn timestamp_from_duration(&self, duration: Duration) -> u32 {
        let seconds: u32 =
            ((duration.as_secs().wrapping_mul(self.freq as u64)) % 2u64.pow(32)) as u32;
        let fract = ((self.freq as u64 * duration.subsec_nanos() as u64) / 1_000_000_000) as u32;
        seconds.wrapping_add(fract)
    }

    pub fn now(&self) -> Result<u32, SystemTimeError> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| self.timestamp_from_duration(d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_from_duration_secs_only() {
        let clock = RtpClock::new(90_000);

        let want = 3780000;
        let got = clock.timestamp_from_duration(Duration::from_secs(42));

        assert_eq!(want, got);
    }

    #[test]
    fn test_timestamp_from_duration_fracts_only() {
        let clock = RtpClock::new(90_000);

        let want = 3780;
        let got = clock.timestamp_from_duration(Duration::from_millis(42));

        assert_eq!(want, got);
    }

    #[test]
    fn test_timestamp_from_duration_both() {
        let clock = RtpClock::new(90_000);

        let want = 3783780;
        let got = clock.timestamp_from_duration(Duration::from_millis(42042));

        assert_eq!(want, got);
    }

    #[test]
    fn test_timestamp_from_duration_wrap() {
        let clock = RtpClock::new(27_000_000);

        let want = 1684748784;
        let got = clock.timestamp_from_duration(Duration::from_millis(1234567891234));

        assert_eq!(want, got);
    }
}
