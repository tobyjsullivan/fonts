const HEADER: [u8; 4] = [0x4F, 0x54, 0x54, 0x4F]; // 'OTTO'
const HEADER_OFFSET: usize = 0;
const HEADER_LENGTH: usize = 4;

pub struct OpenTypeCffFile {}

impl OpenTypeCffFile {
    pub fn detect(content: &Vec<u8>) -> bool {
        &content[HEADER_OFFSET..HEADER_OFFSET + HEADER_LENGTH] == &HEADER
    }
}
