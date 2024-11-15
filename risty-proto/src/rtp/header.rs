use packed_struct::prelude::*;
use rand::{random, rngs::StdRng, thread_rng, Rng, RngCore, SeedableRng};
use risty_core::Marshal;

/// RTP header fields. The first twelve octets are present in every RTP packet, while the
/// list of CSRC identifiers is present only when inserted by a mixer.
//     0                   1                   2                   3
//     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |V=2|P|X|  CC   |M|     PT      |       sequence number         |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |                           timestamp                           |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |           synchronization source (SSRC) identifier            |
//    +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
//    |            contributing source (CSRC) identifiers             |
//    |                             ....                              |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
#[derive(PackedStruct, Debug, PartialEq, Eq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct Header {
    // This field identifies the version of RTP. Always 2.
    #[packed_field(bits = "0..=1")]
    pub version: u8,

    /// If the padding bit is set, the packet contains one or more
    /// additional padding octets at the end which are not part of the
    /// payload. The last octet of the padding contains a count of how
    /// many padding octets should be ignored, including itself.
    #[packed_field(bits = "2")]
    pub padding: bool,

    /// If the extension bit is set, the fixed header MUST be followed by
    /// exactly one header extension.
    #[packed_field(bits = "3")]
    pub extension: bool,

    /// The CSRC count contains the number of CSRC identifiers that follow
    /// the fixed header.
    #[packed_field(bits = "4..=7")]
    pub csrc_count: u8,

    /// The interpretation of the marker is defined by a profile.  It is
    /// intended to allow significant events such as frame boundaries to
    /// be marked in the packet stream. A profile MAY define additional
    /// marker bits or specify that there is no marker bit by changing the
    /// number of bits in the payload type field.
    #[packed_field(bits = "8")]
    pub marker: bool,
    /// This field identifies the format of the RTP payload and determines
    /// its interpretation by the application. A profile MAY specify a
    /// default static mapping of payload type codes to payload formats.
    #[packed_field(bits = "9..=15")]
    pub payload_type: u8,

    /// The sequence number increments by one for each RTP data packet
    /// sent, and may be used by the receiver to detect packet loss and to
    /// restore packet sequence. The initial value of the sequence number
    /// SHOULD be random (unpredictable) to make known-plaintext attacks
    /// on encryption more difficult.
    #[packed_field(bits = "16..=31", endian = "msb")]
    pub sequence_number: u16,

    /// The timestamp reflects the sampling instant of the first octet in
    /// the RTP data packet. The sampling instant MUST be derived from a
    /// clock that increments monotonically and linearly in time to allow
    /// synchronization and jitter calculations. The clock
    /// frequency is dependent on the format of data carried as payload
    /// and is specified statically in the profile or payload format
    /// specification that defines the format, or MAY be specified
    /// dynamically for payload formats defined through non-RTP means.
    #[packed_field(bits = "32..=63", endian = "msb")]
    pub timestamp: u32,

    #[packed_field(bits = "64..95", endian = "msb")]
    pub ssrc: u32,
}

impl Header {
    /// Create a new header with a random sequence number and a random ssrc
    pub fn new(
        payload_type: u8,
        initial_timestamp: u32,
        initial_marker: bool,
        seed: Option<u64>, // do not feed a seed if you want to construct non-deternism PRNG
    ) -> Self {
        let mut rng = if let Some(s) = seed {
            StdRng::seed_from_u64(s)
        } else {
            StdRng::from_entropy()
        };

        Self {
            version: 2,
            padding: false,
            extension: false,
            csrc_count: 0,
            marker: initial_marker,
            payload_type,
            sequence_number: rng.gen(),
            ssrc: rng.gen(),
            timestamp: initial_timestamp,
        }
    }

    /// Upon sending a new packet, one should call this method to update the sequence number (+1 increment),
    /// adjust the timestamp of the packet and possibly the marker bit.
    pub fn update(&mut self, new_timestamp: u32, new_marker: bool) {
        self.sequence_number += 1;
        self.timestamp = new_timestamp;
        self.marker = new_marker;
    }
}

impl Marshal for Header {
    fn marshal(&self, buf: &mut [u8]) -> Result<usize, risty_core::MarshalError> {
        let n = self.marshal_size();
        self.pack_to_slice(&mut buf[0..n])?;
        Ok(n)
    }

    fn marshal_size(&self) -> usize {
        return 12; // we assume no extension, no csrc and no padding for now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let want = Header {
            version: 2,
            padding: false,
            extension: false,
            csrc_count: 0,
            marker: true,
            payload_type: 96,
            sequence_number: 13415,
            timestamp: 0xDEADBEEF,
            ssrc: 3141565102,
        };
        let got = Header::new(96, 0xDEADBEEF, true, Some(0xDEADBEEF5555AAAA));

        assert_eq!(want, got);
    }

    #[test]
    fn test_marshal() {
        let mut buf = [0u8; 12];
        let header = Header::new(18, 160, true, Some(0xDEADBEEF5555AAAA));
        let n = header.marshal(&mut buf).unwrap();

        assert_eq!(n, 12);

        assert_eq!(header.marshal_size(), 12);

        assert_eq!(
            buf,
            [0x80, 0x92, 0x34, 0x67, 0x00, 0x00, 0x00, 0xA0, 0x76, 0x80, 0xF5, 0x5C]
        );
    }

    #[test]
    fn test_update() {
        let mut header = Header::new(18, 160, true, Some(0xDEADBEEF5555AAAA));

        let initial_sequence_number = header.sequence_number;

        header.update(320, false);

        assert_eq!(header.sequence_number, initial_sequence_number + 1);

        assert_eq!(header.timestamp, 320);

        assert_eq!(header.marker, false);
    }
}
