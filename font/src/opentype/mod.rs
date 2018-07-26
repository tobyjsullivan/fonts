const HEADER: [u8; 4] = [0x00u8, 0x01, 0x00, 0x00]; // 0x00010000
const HEADER_OFFSET: usize = 0;
const HEADER_LENGTH: usize = 4;
const NUM_TABLES_OFFSET: usize = 4;
const SEARCH_RANGE_OFFSET: usize = 6;
const ENTRY_SELECTION_OFFSET: usize = 8;
const RANGE_SHIFT_OFFSET: usize = 10;
const TABLE_RECORDS_OFFSET: usize = 12;
const TABLE_RECORD_LENGTH: usize = 16;
const TABLE_TAG_OFFSET: usize = 0;
const TABLE_TAG_LENGTH: usize = 4;
const TABLE_CHECKSUM_OFFSET: usize = 4;

#[derive(Debug)]
pub struct OpenTypeFile {
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    table_records: Vec<TableRecord>,
}

impl OpenTypeFile {
    pub fn deserialize(content: &[u8]) -> Self {
        if !Self::detect(content) {
            panic!("Incorrect file type.");
        }

        Self {
            num_tables: Self::parse_num_tables(content),
            search_range: Self::parse_search_range(content),
            entry_selector: Self::parse_entry_selector(content),
            range_shift: Self::parse_range_shift(content),
            table_records: Self::parse_table_records(content),
        }
    }

    fn detect(content: &[u8]) -> bool {
        &content[HEADER_OFFSET..HEADER_LENGTH] == &HEADER
    }

    fn parse_num_tables(content: &[u8]) -> u16 {
        (content[NUM_TABLES_OFFSET] as u16) << 8 | content[NUM_TABLES_OFFSET+1] as u16
    }

    fn parse_search_range(content: &[u8]) -> u16 {
        (content[SEARCH_RANGE_OFFSET] as u16) << 8 | content[SEARCH_RANGE_OFFSET+1] as u16
    }

    fn parse_entry_selector(content: &[u8]) -> u16 {
        (content[ENTRY_SELECTION_OFFSET] as u16) << 8 | content[ENTRY_SELECTION_OFFSET+1] as u16
    }

    fn parse_range_shift(content: &[u8]) -> u16 {
        (content[RANGE_SHIFT_OFFSET] as u16) << 8 | content[RANGE_SHIFT_OFFSET+1] as u16
    }

    fn parse_table_records(content: &[u8]) -> Vec<TableRecord> {
        let mut records: Vec<TableRecord> = vec![];
        let num_tables: usize = Self::parse_num_tables(content) as usize;
        for n in 0..num_tables {
            records.push(Self::parse_nth_table_record(content, n));
        }
        records
    }

    fn parse_nth_table_record(content: &[u8], n: usize) -> TableRecord {
        let offset = TABLE_RECORDS_OFFSET + n * TABLE_RECORD_LENGTH;
        let record_content: &[u8] = &content[offset..offset+TABLE_RECORD_LENGTH];

        TableRecord::deserialize(record_content)
    }
}

#[derive(Debug)]
struct TableRecord {
    tag: [u8; 4],
    checksum: u32,
}

impl TableRecord {
    fn deserialize(content: &[u8]) -> Self {
        Self {
            tag: Self::parse_tag(content),
            checksum: Self::parse_checksum(content),
        }
    }

    fn parse_tag(content: &[u8]) -> [u8; 4] {
        let tag = &content[TABLE_TAG_OFFSET..TABLE_TAG_OFFSET+TABLE_TAG_LENGTH];
        [tag[0], tag[1], tag[2], tag[3]]
    }

    fn parse_checksum(content: &[u8]) -> u32 {
        bytes_to_u32(&content[TABLE_CHECKSUM_OFFSET..TABLE_CHECKSUM_OFFSET+4])
    }
}

fn bytes_to_u32(bytes: &[u8]) -> u32 {
    if bytes.len() != 4 {
        panic!("Invalid vector: {:?}", bytes);
    }

    let mut out: u32 = (bytes[0] as u32) << 24;
    out |= (bytes[1] as u32) << 16;
    out |= (bytes[2] as u32) << 8;
    out |= bytes[3] as u32;
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_type() {
        let mut content = vec![0x00u8; 47252];
        content[..4].clone_from_slice(&[0x00u8, 0x01, 0x00, 0x00]);
        assert!(OpenTypeFile::detect(&content));
    }

    #[test]
    fn detect_bad_type() {
        let mut content = vec![0x00u8; 47252];
        content[..4].clone_from_slice(&[0x00u8, 0x01, 0x00, 0x01]);
        assert_eq!(OpenTypeFile::detect(&content), false);
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
        assert_eq!(OpenTypeFile::parse_num_tables(&content), 17);
        assert_eq!(OpenTypeFile::parse_search_range(&content), 256);
        assert_eq!(OpenTypeFile::parse_entry_selector(&content), 4);
        assert_eq!(OpenTypeFile::parse_range_shift(&content), 16);
    }

    #[test]
    fn parse_table_records() {
        let mut content = vec![0x00u8; 47252];
        content[5] = 0x12;
        content[12..12+4].clone_from_slice(&[0x01u8, 0x02, 0x03, 0x04]);
        content[12+32..12+32+4].clone_from_slice(&[0x02u8, 0x04, 0x08, 0x10]);

        let table_records = OpenTypeFile::parse_table_records(&content);

        assert_eq!(table_records.len(), 18);
        assert_eq!(table_records[0].tag, [0x01u8, 0x02, 0x03, 0x04]);
        assert_eq!(table_records[2].tag, [0x02u8, 0x04, 0x08, 0x10]);
    }

    #[test]
    fn parse_nth_table_record_first() {
        let mut content = vec![0x00u8; 47252];
        content[12..12+4].clone_from_slice(&[0x01u8, 0x02, 0x03, 0x04]);
        content[16..16+4].clone_from_slice(&[0xFCu8, 0xFD, 0xFE, 0xFF]);

        let rec0 = OpenTypeFile::parse_nth_table_record(&content, 0);
        assert_eq!(rec0.tag, [0x01u8, 0x02, 0x03, 0x04]);
        assert_eq!(rec0.checksum, 0xFCFDFEFF);
    }

    #[test]
    fn parse_nth_table_record_offset() {
        let mut content = vec![0x00u8; 47252];
        content[12+32..12+32+4].clone_from_slice(&[0x02u8, 0x04, 0x08, 0x10]);

        let rec2 = OpenTypeFile::parse_nth_table_record(&content, 2);
        assert_eq!(rec2.tag, [0x02u8, 0x04, 0x08, 0x10]);
    }

    #[test]
    fn deserialize_table_record() {
        let mut content = vec![0x00u8; 16];
        content[0..4].clone_from_slice(&[0x01u8, 0x02, 0x03, 0x04]);
        content[4..8].clone_from_slice(&[0xFCu8, 0xFD, 0xFE, 0xFF]);

        let rec0 = TableRecord::deserialize(&content);
        assert_eq!(rec0.tag, [0x01u8, 0x02, 0x03, 0x04]);
        assert_eq!(rec0.checksum, 0xFCFDFEFF);

    }
}
