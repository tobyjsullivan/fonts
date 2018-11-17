use opentype::tables::name::Encoding;

pub fn lookup(encoding_id: u16) -> Option<Encoding> {
    match encoding_id {
        0 => Some(Encoding::Unicode1),
        1 => Some(Encoding::Unicode11),
        2 => Some(Encoding::ISO10646),
        3 => Some(Encoding::Unicode2BMP),
        4 => Some(Encoding::Unicode2Full),
        5 => Some(Encoding::UnicodeVariation),
        6 => Some(Encoding::UnicodeFull),
        _ => None,
    }
}
