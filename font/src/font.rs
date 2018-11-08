use filetype::FileType;
use opentype;
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
        use opentype::tables::name;
        ot_font
            .name
            .as_ref()
            .and_then(|name_table| {
                name_table
                    .find_strings(field)
                    .iter()
                    .find(|el| match el {
                        (name::Platform::Unicode, name::Encoding::Unicode { encoding: _ }, _) => {
                            true
                        }
                        _ => false,
                    }).map(|el| el.2)
            }).map(|bytes| String::from_utf8_lossy(bytes).into_owned())
    }
}

#[derive(Debug)]
pub enum FontParseErr {
    UnrecognizedFormatError,
}
