mod opentype;

#[derive(Debug)]
pub enum FileType {
    OpenTypeWithTrueTypeOutlines, // https://docs.microsoft.com/en-us/typography/opentype/spec/otff
    OpenTypeWithCFFData, // https://docs.microsoft.com/en-us/typography/opentype/spec/otff
    OpenTypeFontCollection, // https://docs.microsoft.com/en-us/typography/opentype/spec/otff
    PostScriptInSfnt, // https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html
    AppleCompatibleTrueType, // https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html
    Woff, // https://www.w3.org/TR/2012/REC-WOFF-20121213/
    Woff2, // https://www.w3.org/TR/WOFF2/
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
        match content[0..4] {
            [0x00, 0x01, 0x00, 0x00] => FileType::OpenTypeWithTrueTypeOutlines, // 0x00010000
            [0x4F, 0x54, 0x54, 0x4F] => FileType::OpenTypeWithCFFData, // 'OTTO'
            [0x74, 0x74, 0x63, 0x66] => FileType::OpenTypeFontCollection, // 'ttcf'
            [0x74, 0x79, 0x70, 0x31] => FileType::PostScriptInSfnt, // 'typ1'
            [0x74, 0x72, 0x75, 0x65] => FileType::AppleCompatibleTrueType, // 'true'
            [0x77, 0x4F, 0x46, 0x46] => FileType::Woff, // 'wOFF'
            [0x77, 0x4F, 0x46, 0x32] => FileType::Woff2, // 'wOF2'
            _ => panic!("Unrecognised type"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
