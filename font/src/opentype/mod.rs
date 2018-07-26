use byteorder::{ByteOrder, BigEndian};

mod cmap;

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
const TABLE_OFFSET_OFFSET: usize = 8;
const TABLE_LENGTH_OFFSET: usize = 12;

#[derive(Debug)]
pub struct OpenTypeFile {
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    table_records: Vec<TableRecord>,
    cmap: Option<cmap::CmapTable>,
}

impl OpenTypeFile {
    pub fn deserialize(content: &[u8]) -> Self {
        if !Self::detect(content) {
            panic!("Incorrect file type.");
        }

        let table_records = Self::parse_table_records(content);
        let mut cmap_data: Option<&[u8]> = None;
        for record in &table_records {
            match record.table_type() {
                TableType::Cmap => cmap_data = Some(record.table_data(content)),
                TableType::Unknown => {},
            }
        }

        Self {
            num_tables: Self::parse_num_tables(content),
            search_range: Self::parse_search_range(content),
            entry_selector: Self::parse_entry_selector(content),
            range_shift: Self::parse_range_shift(content),
            table_records: table_records,
            cmap: match cmap_data {
                Some(data) => Some(cmap::CmapTable::deserialize(data)),
                None => None,
            },
        }
    }

    fn detect(content: &[u8]) -> bool {
        &content[HEADER_OFFSET..HEADER_LENGTH] == &HEADER
    }

    fn parse_num_tables(content: &[u8]) -> u16 {
        BigEndian::read_u16(&content[NUM_TABLES_OFFSET..NUM_TABLES_OFFSET+2])
    }

    fn parse_search_range(content: &[u8]) -> u16 {
        BigEndian::read_u16(&content[SEARCH_RANGE_OFFSET..SEARCH_RANGE_OFFSET+2])
    }

    fn parse_entry_selector(content: &[u8]) -> u16 {
        BigEndian::read_u16(&content[ENTRY_SELECTION_OFFSET..ENTRY_SELECTION_OFFSET+2])
    }

    fn parse_range_shift(content: &[u8]) -> u16 {
        BigEndian::read_u16(&content[RANGE_SHIFT_OFFSET..RANGE_SHIFT_OFFSET+2])
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

enum TableType {
    Cmap,
    Unknown,
}

#[derive(Debug)]
struct TableRecord {
    tag: [char; 4],
    checksum: u32,
    offset: usize,
    length: usize,
}

impl TableRecord {
    fn deserialize(content: &[u8]) -> Self {
        Self {
            tag: Self::parse_tag(content),
            checksum: Self::parse_checksum(content),
            offset: Self::parse_offset(content),
            length: Self::parse_length(content),
        }
    }

    fn table_type(&self) -> TableType {
        match self.tag {
            ['c' , 'm' , 'a' , 'p'] => TableType::Cmap,
            _ => TableType::Unknown,
        }
    }

    fn table_data<'a>(&self, content: &'a [u8]) -> &'a [u8] {
        &content[self.offset..self.offset+self.length]
    }

    fn parse_tag(content: &[u8]) -> [char; 4] {
        let tag = &content[TABLE_TAG_OFFSET..TABLE_TAG_OFFSET+TABLE_TAG_LENGTH];
        [tag[0] as char, tag[1] as char, tag[2] as char, tag[3] as char]
    }

    fn parse_checksum(content: &[u8]) -> u32 {
        BigEndian::read_u32(&content[TABLE_CHECKSUM_OFFSET..TABLE_CHECKSUM_OFFSET+4])
    }

    fn parse_offset(content: &[u8]) -> usize {
        BigEndian::read_u32(&content[TABLE_OFFSET_OFFSET..TABLE_OFFSET_OFFSET+4]) as usize
    }

    fn parse_length(content: &[u8]) -> usize {
        BigEndian::read_u32(&content[TABLE_LENGTH_OFFSET..TABLE_LENGTH_OFFSET+4]) as usize
    }
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
        content[12..12+4].clone_from_slice(&[0x6eu8, 0x61, 0x6d, 0x65]);
        content[12+32..12+32+4].clone_from_slice(&[0x67u8, 0x6c, 0x79, 0x66]);

        let table_records = OpenTypeFile::parse_table_records(&content);

        assert_eq!(table_records.len(), 18);
        assert_eq!(table_records[0].tag, ['n', 'a', 'm', 'e']);
        assert_eq!(table_records[2].tag, ['g', 'l', 'y', 'f']);
    }

    #[test]
    fn parse_nth_table_record_first() {
        let mut content = vec![0x00u8; 47252];
        content[12..12+4].clone_from_slice(&[0x6eu8, 0x61, 0x6d, 0x65]);
        content[16..16+4].clone_from_slice(&[0xFCu8, 0xFD, 0xFE, 0xFF]);

        let rec0 = OpenTypeFile::parse_nth_table_record(&content, 0);
        assert_eq!(rec0.tag, ['n', 'a', 'm', 'e']);
        assert_eq!(rec0.checksum, 0xFCFDFEFF);
    }

    #[test]
    fn parse_nth_table_record_offset() {
        let mut content = vec![0x00u8; 47252];
        content[12+32..12+32+4].clone_from_slice(&[0x6eu8, 0x61, 0x6d, 0x65]);

        let rec2 = OpenTypeFile::parse_nth_table_record(&content, 2);
        assert_eq!(rec2.tag, ['n', 'a', 'm', 'e']);
    }

    #[test]
    fn deserialize_table_record() {
        let mut content = vec![0x00u8; 16];
        content[0..4].clone_from_slice(&[0x6eu8, 0x61, 0x6d, 0x65]);
        content[4..8].clone_from_slice(&[0xFCu8, 0xFD, 0xFE, 0xFF]);
        content[8..12].clone_from_slice(&[0x00u8, 0x00, 0xFF, 0xDD]);
        content[12..16].clone_from_slice(&[0x00u8, 0x00, 0x08, 0x00]);

        let rec0 = TableRecord::deserialize(&content);
        assert_eq!(rec0.tag, ['n', 'a', 'm', 'e']);
        assert_eq!(rec0.checksum, 0xFCFDFEFF);
        assert_eq!(rec0.offset, 0x0000FFDD);
        assert_eq!(rec0.length, 0x00000800);
    }

    #[test]
    fn table_record_data() {
        let mut content = vec![0x00u8; 16];
        content[0..4].clone_from_slice(&[0x6eu8, 0x61, 0x6d, 0x65]);
        content[4..8].clone_from_slice(&[0xFCu8, 0xFD, 0xFE, 0xFF]);
        content[8..12].clone_from_slice(&[0x00u8, 0x00, 0x00, 0x02]);
        content[12..16].clone_from_slice(&[0x00u8, 0x00, 0x00, 0x04]);

        let rec0 = TableRecord::deserialize(&content);

        let table_data = rec0.table_data(&content);
        assert_eq!(table_data, &[0x6du8, 0x65, 0xFC, 0xFD]);
    }
}
