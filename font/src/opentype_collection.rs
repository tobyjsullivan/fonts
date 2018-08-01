const HEADER: [u8; 4] = [0x74, 0x74, 0x63, 0x66]; // 'ttcf'
const HEADER_OFFSET: usize = 0;
const HEADER_LENGTH: usize = 4;

pub struct OpenTypeCollectionFile {}

impl OpenTypeCollectionFile {
    pub fn detect(content: &Vec<u8>) -> bool {
        &content[HEADER_OFFSET..HEADER_LENGTH] == &HEADER
    }
}