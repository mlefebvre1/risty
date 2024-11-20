use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub struct NtpTimestamp {
    pub seconds: u32,
    pub fraction: u32,
}

impl From<Duration> for NtpTimestamp {
    fn from(value: Duration) -> Self {
        let seconds = value.as_secs() as u32;
        let fraction = value.subsec_nanos();

        Self { seconds, fraction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_duration_to_timestamp() {
        let want = NtpTimestamp {
            seconds: 45,
            fraction: 5_000_000,
        };
        let got = NtpTimestamp::from(Duration::from_millis(45005));

        assert_eq!(got, want)
    }

    #[test]
    fn test_duration_no_secs_to_timestamp() {
        let want = NtpTimestamp {
            seconds: 0,
            fraction: 42,
        };

        let got = NtpTimestamp::from(Duration::from_nanos(42));

        assert_eq!(got, want);
    }

    #[test]
    fn test_duration_no_nanos_to_timestamp() {
        let want = NtpTimestamp {
            seconds: 42,
            fraction: 0,
        };

        let got = NtpTimestamp::from(Duration::from_secs(42));

        assert_eq!(got, want);
    }
}
