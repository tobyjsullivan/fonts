const HEADER: [u8; 4] = [0x74, 0x79, 0x70, 0x31]; // 'typ1'
const HEADER_OFFSET: usize = 0;
const HEADER_LENGTH: usize = 4;

pub struct PostScriptFile {}

impl PostScriptFile {
    pub fn detect(content: &Vec<u8>) -> bool {
        &content[HEADER_OFFSET..HEADER_LENGTH] == &HEADER
    }
}