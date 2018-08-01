const HEADER: [u8; 4] = [0x74, 0x72, 0x75, 0x65]; // 'true'
const HEADER_OFFSET: usize = 0;
const HEADER_LENGTH: usize = 4;

pub struct TrueTypeFile {}

impl TrueTypeFile {
    pub fn detect(content: &Vec<u8>) -> bool {
        &content[HEADER_OFFSET..HEADER_LENGTH] == &HEADER
    }
}