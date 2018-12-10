use opentype::encoding::Encoding;

pub fn lookup(encoding_id: u16) -> Option<Encoding> {
    match encoding_id {
        0 => Some(Encoding::WindowsSymbol),
        1 => Some(Encoding::WindowsUnicodeBMP),
        2 => Some(Encoding::WindowsShiftJIS),
        3 => Some(Encoding::WindowsPRC),
        4 => Some(Encoding::WindowsBig5),
        5 => Some(Encoding::WindowsWansung),
        6 => Some(Encoding::WindowsJohab),
        10 => Some(Encoding::WindowsUnicodeUCS4),
        _ => None,
    }
}
