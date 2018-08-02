const HEADER_V1: [u8; 4] = [0x00, 0x00, 0x01, 0x00]; // '0x00010000' (little endian)
const HEADER_V2: [u8; 4] = [0x01, 0x00, 0x02, 0x00]; // '0x00020001' (little endian)
const HEADER_V2_2: [u8; 4] = [0x02, 0x00, 0x02, 0x00]; // '0x00020002' (little endian)
const HEADER_OFFSET: usize = 8;
const HEADER_LENGTH: usize = 4;

pub struct EmbeddedOpenTypeFile {}

impl EmbeddedOpenTypeFile {
    pub fn detect(content: &Vec<u8>) -> bool {
        &content[HEADER_OFFSET..HEADER_OFFSET+HEADER_LENGTH] == &HEADER_V1 ||
            &content[HEADER_OFFSET..HEADER_OFFSET+HEADER_LENGTH] == &HEADER_V2 ||
            &content[HEADER_OFFSET..HEADER_OFFSET+HEADER_LENGTH] == &HEADER_V2_2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_type() {
        let mut content = vec![0x00u8; 47252];
        content[..12].clone_from_slice(&[0x3c, 0x11, 0x00, 0x00, 0x58, 0x10, 0x00, 0x00, 0x01, 0x00, 0x02, 0x00]);
        assert!(EmbeddedOpenTypeFile::detect(&content));
    }
}