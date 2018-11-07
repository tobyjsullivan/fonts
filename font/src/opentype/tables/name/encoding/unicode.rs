#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnicodeEncoding {
    Unicode1,         // Deprecated, Unicode 1.0
    Unicode11,        // Deprecated, Unicode 1.1
    ISO,              // Deprecated, ISO/IEC 10646
    Unicode2BMP,      // Unicode 2.0, BMP only (cmap subtable formats 0, 4, 6)
    Unicode2Full,     // Unicode 2.0, full repertoire (cmap subtable formats 0, 4, 6, 10, 12)
    UnicodeVariation, // Unicode Variation Sequences (cmap subtable format 14)
    UnicodeFull,      // Unicode full repertoire (cmap subtable formats 0, 4, 6, 10, 12, 13)
}

impl UnicodeEncoding {
    pub fn lookup(encoding_id: u16) -> Option<UnicodeEncoding> {
        match encoding_id {
            0 => Some(UnicodeEncoding::Unicode1),
            1 => Some(UnicodeEncoding::Unicode11),
            2 => Some(UnicodeEncoding::ISO),
            3 => Some(UnicodeEncoding::Unicode2BMP),
            4 => Some(UnicodeEncoding::Unicode2Full),
            5 => Some(UnicodeEncoding::UnicodeVariation),
            6 => Some(UnicodeEncoding::UnicodeFull),
            _ => None,
        }
    }
}
