use std::time::{Duration, Instant, SystemTime, SystemTimeError, UNIX_EPOCH};

use ringbuffer::{AllocRingBuffer, RingBuffer};
use risty_proto::risty_core::{Marshal, MarshalError, RtpClock};
use risty_proto::rtp;

pub struct Builder {
    pt: u8,
    clock_freq: u32,
    buffer_size: u32,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            pt: 96,
            clock_freq: 90_000,
            buffer_size: 1024,
        }
    }
}

impl Builder {
    pub fn set_payload_type(mut self, pt: u8) -> Self {
        self.pt = pt;
        self
    }

    pub fn set_clock_frequency(mut self, freq: u32) -> Self {
        self.clock_freq = freq;
        self
    }

    pub fn set_buffer_size(mut self, buffer_size: u32) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    pub fn build(self) -> Result<RtpSender, SystemTimeError> {
        let clock = RtpClock::new(self.clock_freq);
        let header = rtp::Header::new(
            self.pt,
            clock.timestamp_from_duration(SystemTime::now().duration_since(UNIX_EPOCH)?),
            false,
            None,
        );
        Ok(RtpSender {
            header: header,
            clock: clock,
            buffer: AllocRingBuffer::new(self.buffer_size as usize),
        })
    }
}

pub struct RtpSender {
    header: rtp::Header,
    clock: RtpClock,
    buffer: AllocRingBuffer<rtp::Packet>,
}

pub struct RtpTransmit {
    pub remote_sockaddr: String,
    pub buf_size: usize,
}

impl RtpSender {
    pub fn new() -> Builder {
        Builder::default()
    }

    // more like now_epoch
    pub fn push_to_queue(&mut self, payload: Vec<u8>, now: Duration, new_marker: bool) {
        self.header
            .update(self.clock.timestamp_from_duration(now), new_marker);
        let packet = rtp::Packet::new(self.header.clone(), payload);
        // probably need to do some time checking

        self.buffer.push(packet);
    }

    pub fn poll_transmit(
        &mut self,
        send_buf: &mut [u8],
        _now: Duration,
    ) -> Result<usize, risty_proto::risty_core::MarshalError> {
        // normally we would probably get by index using the now variable
        if let Some(packet) = self.buffer.dequeue() {
            return packet.marshal(send_buf);
        }
        Ok(0) // todo make an error
    }
}
