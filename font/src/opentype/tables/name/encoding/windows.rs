#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowsEncoding {
    Symbol,
    UnicodeBMP, // UCS-2
    ShiftJIS,
    PRC,
    Big5,
    Wansung,
    Johab,
    UnicodeUCS4,
}

impl WindowsEncoding {
    pub fn lookup(encoding_id: u16) -> Option<WindowsEncoding> {
        match encoding_id {
            0 => Some(WindowsEncoding::Symbol),
            1 => Some(WindowsEncoding::UnicodeBMP),
            2 => Some(WindowsEncoding::ShiftJIS),
            3 => Some(WindowsEncoding::PRC),
            4 => Some(WindowsEncoding::Big5),
            5 => Some(WindowsEncoding::Wansung),
            6 => Some(WindowsEncoding::Johab),
            10 => Some(WindowsEncoding::UnicodeUCS4),
            _ => None,
        }
    }
}
