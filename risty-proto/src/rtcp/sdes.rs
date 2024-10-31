use super::header::{Header, PacketType, VERSION};
struct Sdes {
    pub header: Header,

    /// Chunk consists of an SSRC/CSRC identifier followed by a list of zero or more items,
    /// which carry information about the SSRC/CSRC.
    /// * Note: typically, SDES packets can contain 1 or more chunks, but in RIST this is fixed to 1.
    pub chunk: Chunk,
}

impl Sdes {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            header: sdes_header(calculate_sdes_length(chunk.name_length)),
            chunk,
        }
    }
}

fn sdes_header(length: u16) -> Header {
    Header {
        version: VERSION,
        padding: false,
        packet_specific: 1,
        packet_type: PacketType::Sdes as u8,
        length,
    }
}

/// The length of this RTCP packet in 32-bit words minus one, including the header and any padding
fn calculate_sdes_length(name_length: u8) -> u16 {
    //Sizes in bytes
    const HEADER_SIZE: u8 = 4;
    const CHUNK_HEADER_SIZE: u8 = 6;

    let size = HEADER_SIZE + CHUNK_HEADER_SIZE + name_length;

    // 32-bit words minus one, so an integer division will include any padding here
    (size / 4) as u16
}

pub struct Chunk {
    /// The synchronization source identifier for the originator of this SDES packet
    pub ssrc: u32,

    /// Identifies this item as a CNAME. Always set to 1.
    pub cname: u8,

    /// Length, in bytes of the user and domain name field.
    pub name_length: u8,

    /// The user and domain name is an ASCII string, which is not null-terminated as the length is specified in the previous field.
    /// RFC 3550 recommends that this string be programmatically generated in the form of “user@host”. RIST implementations are free
    /// to use this field as they see fit.
    pub user_and_domain: String,
}

impl Chunk {
    pub fn new(ssrc: u32, user_and_domain: String) -> Self {
        Self {
            ssrc,
            cname: 1,
            name_length: user_and_domain.len() as u8, //ASCII characters only so this will return the same as .chars().count()
            user_and_domain,
        }
    }
}
