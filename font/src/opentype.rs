const HEADER: [u8; 4] = [0x00u8, 0x01, 0x00, 0x00]; // 0x00010000
const OFFSET_HEADER: usize = 0;
const LEN_HEADER: usize = 4;
const OFFSET_NUM_TABLES: usize = 4;
const OFFSET_SEARCH_RANGE: usize = 6;
const OFFSET_ENTRY_SELECTOR: usize = 8;
const OFFSET_RANGE_SHIFT: usize = 10;

#[derive(Debug)]
pub struct OpenTypeFile {
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
}

impl OpenTypeFile {    
    pub fn from(content: &Vec<u8>) -> Self {
        if !detect(content) {
            panic!("Incorrect file type.");
        }
        
        OpenTypeFile {
            num_tables: parse_num_tables(content),
            search_range: parse_search_range(content),
            entry_selector: parse_entry_selector(content),
            range_shift: parse_range_shift(content),
        }
    }
}

fn detect(content: &Vec<u8>) -> bool {
    &content[OFFSET_HEADER..LEN_HEADER] == &HEADER
}

fn parse_num_tables(content: &Vec<u8>) -> u16 {
    (content[OFFSET_NUM_TABLES] as u16) << 8 | content[OFFSET_NUM_TABLES+1] as u16
}

fn parse_search_range(content: &Vec<u8>) -> u16 {
    (content[OFFSET_SEARCH_RANGE] as u16) << 8 | content[OFFSET_SEARCH_RANGE+1] as u16
}

fn parse_entry_selector(content: &Vec<u8>) -> u16 {
    (content[OFFSET_ENTRY_SELECTOR] as u16) << 8 | content[OFFSET_ENTRY_SELECTOR+1] as u16
}

fn parse_range_shift(content: &Vec<u8>) -> u16 {
    (content[OFFSET_RANGE_SHIFT] as u16) << 8 | content[OFFSET_RANGE_SHIFT+1] as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_type() {
        let mut content = vec![0x00u8; 47252];
        content[..4].clone_from_slice(&[0x00u8, 0x01, 0x00, 0x00]);
        assert!(detect(&content));
    }

    #[test]
    fn detect_bad_type() {
        let mut content = vec![0x00u8; 47252];
        content[..4].clone_from_slice(&[0x00u8, 0x01, 0x00, 0x01]);
        assert_eq!(detect(&content), false);
    }

    #[test]
    fn parse_header() {
        let mut content = vec![0x00u8; 47252];
        content[..12].clone_from_slice(&[
            0x00u8, 0x01, 0x00, 0x00,
            0x00, 0x11,
            0x01, 0x00,
            0x00, 0x04,
            0x00, 0x10]);
        assert_eq!(parse_num_tables(&content), 17);
        assert_eq!(parse_search_range(&content), 256);
        assert_eq!(parse_entry_selector(&content), 4);
        assert_eq!(parse_range_shift(&content), 16);
    }
}
