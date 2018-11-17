use opentype::tables::name::Encoding;

pub fn lookup(encoding_id: u16) -> Option<Encoding> {
    match encoding_id {
        0 => Some(Encoding::Ascii),
        1 => Some(Encoding::ISO10646),
        2 => Some(Encoding::ISO8859_1),
        _ => None,
    }
}
