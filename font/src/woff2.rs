const HEADER: [u8; 4] = [0x77, 0x4F, 0x46, 0x32]; // 'wOF2'
const HEADER_OFFSET: usize = 0;
const HEADER_LENGTH: usize = 4;

pub struct Woff2File {}

impl Woff2File {
    pub fn detect(content: &Vec<u8>) -> bool {
        &content[HEADER_OFFSET..HEADER_OFFSET + HEADER_LENGTH] == &HEADER
    }
}
