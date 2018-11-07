#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOEncoding {
    Ascii,     // 7-bit ASCII
    Iso10646,  // ISO 10646
    Iso8859_1, // ISO 8859-1
}

impl ISOEncoding {
    pub fn lookup(encoding_id: u16) -> Option<ISOEncoding> {
        match encoding_id {
            0 => Some(ISOEncoding::Ascii),
            1 => Some(ISOEncoding::Iso10646),
            2 => Some(ISOEncoding::Iso8859_1),
            _ => None,
        }
    }
}
