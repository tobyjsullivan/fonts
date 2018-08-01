extern crate byteorder;

mod opentype;
mod opentype_cff;
mod opentype_collection;
mod postscript;
mod truetype_apple;
mod woff;
mod woff2;
mod embedded_opentype;

#[derive(Debug)]
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

impl Font {
    pub fn from(content: &Vec<u8>) -> Self {
        let file_type = Font::detect_type(content);

        match file_type {
            FileType::OpenTypeWithTrueTypeOutlines => {
                let parsed = opentype::OpenTypeFile::deserialize(content);
                println!("OpenTypeFile: {:?}", parsed);
            },
            _ => {},
        }

        Font {
            file_type,
        }
    }

    fn detect_type(content: &Vec<u8>) -> FileType {
        if opentype::OpenTypeFile::detect(content) {
            FileType::OpenTypeWithTrueTypeOutlines
        } else if opentype_cff::OpenTypeCffFile::detect(content) {
            FileType::OpenTypeWithCFFData
        } else if opentype_collection::OpenTypeCollectionFile::detect(content) {
            FileType::OpenTypeFontCollection
        } else if postscript::PostScriptFile::detect(content) {
            FileType::PostScriptInSfnt
        } else if truetype_apple::TrueTypeFile::detect(content) {
            FileType::AppleCompatibleTrueType
        } else if woff::WoffFile::detect(content) {
            FileType::Woff
        } else if woff2::Woff2File::detect(content) {
            FileType::Woff2
        } else if embedded_opentype::EmbeddedOpenTypeFile::detect(content) {
            FileType::EmbeddedOpenType
        } else {
            panic!("Unrecognised type")
        }
    }
}
