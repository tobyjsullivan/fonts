use filetype::FileType;
use opentype;
use strings;
use truetype;
use Name;

#[derive(Debug)]
enum ParsedFont<'a> {
    OpenType(opentype::OpenTypeFile<'a>),
    TrueType(truetype::TrueTypeFile<'a>),
    None,
}

#[derive(Debug)]
pub struct Font<'a> {
    pub file_type: FileType,
    font: ParsedFont<'a>,
}

impl<'a> Font<'a> {
    pub fn from_bytes(content: &'a [u8]) -> Result<Self, FontParseErr> {
        match FileType::detect(content) {
            Some(file_type) => {
                let font = match file_type {
                    FileType::OpenTypeWithTrueTypeOutlines | FileType::OpenTypeWithCFFData => {
                        ParsedFont::OpenType(opentype::OpenTypeFile::deserialize(content))
                    }
                    FileType::AppleCompatibleTrueType => {
                        ParsedFont::TrueType(truetype::TrueTypeFile::deserialize(content))
                    }
                    _ => ParsedFont::None,
                };

                Ok(Font { file_type, font })
            }
            None => Err(FontParseErr::UnrecognizedFormatError),
        }
    }

    pub fn dump_glyphs(&self) {
        match &self.font {
            ParsedFont::OpenType(font) => {
                let glyph_count = font.num_glyphs();
                glyph_count.map(|n| {
                    println!("Num glyphs: {}", n);
                    for idx in 0..n {
                        let glyph = font.lookup_glyph(idx as usize);
                        println!("DUMP: {} {:?}", idx, glyph);
                    }
                });
            }
            _ => panic!("Dump not implemented."),
        }
    }

    pub fn available_strings(&self) -> Vec<(Name, String)> {
        match &self.font {
            ParsedFont::OpenType(ot_font) => {
                let mut strings = ot_font
                    .name
                    .as_ref()
                    .map(|name| {
                        name.available_strings()
                            .iter()
                            .map(|(name, enc, bytes)| (name, parse_string(*enc, bytes)))
                            .filter(|(_, o_str)| o_str.is_some())
                            .map(|(n, o_str)| (*n, o_str.unwrap()))
                            .collect()
                    }).unwrap_or(Vec::new());
                strings.sort_unstable();
                strings.dedup();
                strings
            }
            _ => Vec::new(),
        }
    }

    pub fn read_unicode_string(&self, field: Name) -> Option<String> {
        match &self.font {
            ParsedFont::OpenType(ot_font) => Self::read_opentype_string(ot_font, field),
            _ => None,
        }
    }

    fn read_opentype_string(
        ot_font: &opentype::OpenTypeFile,
        field: opentype::tables::name::Name,
    ) -> Option<String> {
        ot_font.name.as_ref().and_then(|name_table| {
            name_table
                .find_strings(field)
                .iter()
                .map(|el| parse_string(el.0, el.1))
                // Filter out empty strings.
                .filter(|o_str| o_str.as_ref().filter(|s| *s != "").is_some())
                .find(|o_str| o_str.is_some())
                .and_then(|o_str| o_str)
        })
    }
}

fn parse_string(encoding: opentype::encoding::Encoding, bytes: &[u8]) -> Option<String> {
    use opentype::encoding::Encoding;

    match encoding {
        Encoding::Unicode1 | Encoding::Unicode2BMP | Encoding::WindowsUnicodeBMP => {
            Some(to_string(to_utf8(strings::Ucs2::from_bytes(bytes))))
        }
        Encoding::UnicodeFull => Some(to_string(strings::Utf8::from_bytes(bytes))),
        Encoding::MacintoshRoman => {
            let mac_str = to_string(to_utf8(strings::AppleRoman::from_bytes(bytes)));
            // Convert Apple line endings (\r) to unix (\n).
            let unix_str = mac_str.replace("\r", "\n");
            Some(unix_str)
        }
        _ => None,
    }
}

fn to_utf8<T: Into<strings::Utf8>>(string: T) -> strings::Utf8 {
    string.into()
}

fn to_string(utf8: strings::Utf8) -> String {
    String::from_utf8(utf8.to_bytes().to_vec()).unwrap()
}

#[derive(Debug)]
pub enum FontParseErr {
    UnrecognizedFormatError,
}
