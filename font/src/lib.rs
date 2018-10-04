extern crate byteorder;

mod embedded_opentype;
mod opentype;
mod opentype_cff;
mod opentype_collection;
mod postscript;
pub mod sfnt;
mod truetype_apple;
mod woff;
mod woff2;

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
    None,
}

impl<'a> Font<'a> {
    pub fn from(content: &'a Vec<u8>) -> Result<Self, FontParseErr> {
        match Font::detect_type(content) {
            Some(file_type) => {
                let mut font = ParsedFont::None;
                if file_type == FileType::OpenTypeWithTrueTypeOutlines {
                    let parsed = opentype::OpenTypeFile::deserialize(content);
                    font = ParsedFont::OpenType(parsed);
                }

                Ok(Font { file_type, font })
            }
            None => Err(FontParseErr::UnrecognizedFormatError),
        }
    }

    fn detect_type(content: &Vec<u8>) -> Option<FileType> {
        if opentype::OpenTypeFile::detect(content) {
            Some(FileType::OpenTypeWithTrueTypeOutlines)
        } else if opentype_cff::OpenTypeCffFile::detect(content) {
            Some(FileType::OpenTypeWithCFFData)
        } else if opentype_collection::OpenTypeCollectionFile::detect(content) {
            Some(FileType::OpenTypeFontCollection)
        } else if postscript::PostScriptFile::detect(content) {
            Some(FileType::PostScriptInSfnt)
        } else if truetype_apple::TrueTypeFile::detect(content) {
            Some(FileType::AppleCompatibleTrueType)
        } else if woff::WoffFile::detect(content) {
            Some(FileType::Woff)
        } else if woff2::Woff2File::detect(content) {
            Some(FileType::Woff2)
        } else if embedded_opentype::EmbeddedOpenTypeFile::detect(content) {
            Some(FileType::EmbeddedOpenType)
        } else {
            None
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
            Font::detect_type(&content),
            Some(FileType::OpenTypeWithTrueTypeOutlines)
        );
    }
}
