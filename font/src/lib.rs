#[derive(Debug)]
pub enum FileType {
    OpenType,
}

#[derive(Debug)]
pub struct Font {
    pub file_type: FileType,
}

impl Font {
    pub fn from(content: &Vec<u8>) -> Self {
        Font {
            file_type: Font::detect_type(content),
        }
    }

    fn detect_type(content: &Vec<u8>) -> FileType {
        match content[0..4] {
            [0x00, 0x01, 0x00, 0x00] => FileType::OpenType,
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
