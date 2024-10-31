use super::header::{Header, PacketType, VERSION};
struct Sdes {
    pub header: Header,

    /// Identifies this item as a CNAME. Always set to 1.
    pub cname: u8,

    /// Length, in bytes of the user and domain name field.
    pub name_length: u8,

    /// The user and domain name is an ASCII string, which is not null-terminated as the length is specified in the previous field.
    /// RFC 3550 recommends that this string be programmatically generated in the form of “user@host”. RIST implementations are free
    /// to use this field as they see fit.
    pub user_and_domain: String,
}

impl Default for Sdes {
    fn default() -> Self {
        Self {
            header: Header {
                version: VERSION,
                padding: false,
                packet_specific: 1,
                packet_type: PacketType::Sdes as u8,
                // The length of this RTCP packet in 32-bit words minus one, including the header and any padding
                length: 0,
                ssrc: 0,
            },
            cname: 1,
            name_length: 0,
            user_and_domain: "".to_string(),
        }
    }
}
