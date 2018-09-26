const HEADER: [u8; 4] = [0x77, 0x4F, 0x46, 0x46]; // 'wOFF'
const HEADER_OFFSET: usize = 0;
const HEADER_LENGTH: usize = 4;

pub struct WoffFile {}

impl WoffFile {
    pub fn detect(content: &Vec<u8>) -> bool {
        &content[HEADER_OFFSET..HEADER_OFFSET + HEADER_LENGTH] == &HEADER
    }
}
