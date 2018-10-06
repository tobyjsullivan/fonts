extern crate byteorder;
extern crate fixed;

mod opentype;
pub mod sfnt;
mod truetype;

#[derive(Debug, PartialEq)]
pub enum FileType {
    /// [https://docs.microsoft.com/en-us/typography/opentype/spec/otff](https://docs.microsoft.com/en-us/typography/opentype/spec/otff)
    OpenTypeWithTrueTypeOutlines,
    /// [https://docs.microsoft.com/en-us/typography/opentype/spec/otff](https://docs.microsoft.com/en-us/typography/opentype/spec/otff)
    OpenTypeWithCFFData,
    /// [https://docs.microsoft.com/en-us/typography/opentype/spec/otff](https://docs.microsoft.com/en-us/typography/opentype/spec/otff)
    OpenTypeFontCollection,
    /// [https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html)
    PostScriptInSfnt,
    /// [https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html)
    AppleCompatibleTrueType,
    /// [https://www.w3.org/TR/2012/REC-WOFF-20121213/](https://www.w3.org/TR/2012/REC-WOFF-20121213/)
    Woff,
    /// [https://www.w3.org/TR/WOFF2/](https://www.w3.org/TR/WOFF2/)
    Woff2,
    /// [https://www.w3.org/Submission/EOT/#FileFormat](https://www.w3.org/Submission/EOT/#FileFormat)
    EmbeddedOpenType,
}

impl FileType {
    const SFNT_HEADER_OFFSET: usize = 0;
    const SFNT_HEADER_LENGTH: usize = 4;
    const EOT_HEADER_OFFSET: usize = 8;
    const EOT_HEADER_LENGTH: usize = 4;

    pub(crate) fn detect(content: &Vec<u8>) -> Option<Self> {
        let sfnt_result = Self::detect_sfnt(content);
        sfnt_result.or_else(|| Self::detect_eot(content))
    }

    fn detect_sfnt(content: &Vec<u8>) -> Option<Self> {
        let sfnt_header =
            &content[Self::SFNT_HEADER_OFFSET..Self::SFNT_HEADER_OFFSET + Self::SFNT_HEADER_LENGTH];
        match sfnt_header {
            // 0x00010000
            &[0x00u8, 0x01, 0x00, 0x00] => Some(FileType::OpenTypeWithTrueTypeOutlines),
            // 'OTTO'
            &[0x4F, 0x54, 0x54, 0x4F] => Some(FileType::OpenTypeWithCFFData),
            // 'ttcf'
            &[0x74, 0x74, 0x63, 0x66] => Some(FileType::OpenTypeFontCollection),
            // 'typ1'
            &[0x74, 0x79, 0x70, 0x31] => Some(FileType::PostScriptInSfnt),
            // 'true'
            &[0x74, 0x72, 0x75, 0x65] => Some(FileType::AppleCompatibleTrueType),
            // 'wOFF'
            &[0x77, 0x4F, 0x46, 0x46] => Some(FileType::Woff),
            // 'wOF2'
            &[0x77, 0x4F, 0x46, 0x32] => Some(FileType::Woff2),
            _ => None,
        }
    }

    fn detect_eot(content: &Vec<u8>) -> Option<Self> {
        let eot_header =
            &content[Self::EOT_HEADER_OFFSET..Self::EOT_HEADER_OFFSET + Self::EOT_HEADER_LENGTH];
        match eot_header {
            // '0x00010000' (little endian)
            &[0x00, 0x00, 0x01, 0x00] |
            // '0x00020001' (little endian)
            &[0x01, 0x00, 0x02, 0x00] |
            // '0x00020002' (little endian)
            &[0x02, 0x00, 0x02, 0x00] => Some(FileType::EmbeddedOpenType),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Font<'a> {
    pub file_type: FileType,
    font: ParsedFont<'a>,
}

#[derive(Debug)]
pub enum FontParseErr {
    UnrecognizedFormatError,
}

#[derive(Debug)]
enum ParsedFont<'a> {
    OpenType(opentype::OpenTypeFile<'a>),
    TrueType(truetype::TrueTypeFile<'a>),
    None,
}

impl<'a> Font<'a> {
    pub fn from(content: &'a Vec<u8>) -> Result<Self, FontParseErr> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_type() {
        let mut content = vec![0x00u8; 47252];
        content[..4].clone_from_slice(&[0x00u8, 0x01, 0x00, 0x00]);
        assert_eq!(
            FileType::detect(&content),
            Some(FileType::OpenTypeWithTrueTypeOutlines)
        );
    }
}
