extern crate byteorder;

mod opentype;
mod opentype_cff;
mod opentype_collection;
mod postscript;
mod truetype_apple;
mod woff;
mod woff2;
mod embedded_opentype;

#[derive(Debug, PartialEq)]
pub enum FileType {
    OpenTypeWithTrueTypeOutlines, // https://docs.microsoft.com/en-us/typography/opentype/spec/otff
    OpenTypeWithCFFData, // https://docs.microsoft.com/en-us/typography/opentype/spec/otff
    OpenTypeFontCollection, // https://docs.microsoft.com/en-us/typography/opentype/spec/otff
    PostScriptInSfnt, // https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html
    AppleCompatibleTrueType, // https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html
    Woff, // https://www.w3.org/TR/2012/REC-WOFF-20121213/
    Woff2, // https://www.w3.org/TR/WOFF2/
    EmbeddedOpenType, // https://www.w3.org/Submission/EOT/#FileFormat
}

#[derive(Debug)]
pub struct Font {
    pub file_type: FileType,
}

#[derive(Debug)]
pub enum FontParseErr {
    UnrecognizedFormatError
}

impl Font {
    pub fn from(content: &Vec<u8>) -> Result<Self, FontParseErr> {
        match Font::detect_type(content) {
            Some(file_type) => {
                if file_type == FileType::OpenTypeWithTrueTypeOutlines {
                    let parsed = opentype::OpenTypeFile::deserialize(content);
                    println!("OpenTypeFile: {:?}", parsed);
                }

                Ok(Font {
                    file_type,
                })
            },
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
        assert_eq!(Font::detect_type(&content), Some(FileType::OpenTypeWithTrueTypeOutlines));
    }
}