use byteorder::{BigEndian, ByteOrder};
use std::fmt;

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
pub(crate) struct SfntFile<'a> {
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    pub table_records: Vec<TableRecord<'a>>,
}

impl<'a> SfntFile<'a> {
    pub fn deserialize(content: &'a [u8]) -> Self {
        Self {
            num_tables: Self::parse_num_tables(content),
            search_range: Self::parse_search_range(content),
            entry_selector: Self::parse_entry_selector(content),
            range_shift: Self::parse_range_shift(content),
            table_records: Self::parse_table_records(content),
        }
    }

    fn parse_num_tables(content: &[u8]) -> u16 {
        BigEndian::read_u16(&content[NUM_TABLES_OFFSET..NUM_TABLES_OFFSET + 2])
    }

    fn parse_search_range(content: &[u8]) -> u16 {
        BigEndian::read_u16(&content[SEARCH_RANGE_OFFSET..SEARCH_RANGE_OFFSET + 2])
    }

    fn parse_entry_selector(content: &[u8]) -> u16 {
        BigEndian::read_u16(&content[ENTRY_SELECTION_OFFSET..ENTRY_SELECTION_OFFSET + 2])
    }

    fn parse_range_shift(content: &[u8]) -> u16 {
        BigEndian::read_u16(&content[RANGE_SHIFT_OFFSET..RANGE_SHIFT_OFFSET + 2])
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
        let record_content: &[u8] = &content[offset..offset + TABLE_RECORD_LENGTH];

        TableRecord::deserialize(record_content, content)
    }
}

pub(crate) struct TableRecord<'a> {
    pub tag: [char; 4],
    checksum: u32,
    offset: usize,
    length: usize,
    pub table_data: &'a [u8],
}

impl<'a> fmt::Debug for TableRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TableRecord {{ tag: {:?}, size: {} bytes }}",
            self.tag, self.length
        )
    }
}

impl<'a> TableRecord<'a> {
    fn deserialize(record_content: &'a [u8], file_content: &'a [u8]) -> Self {
        let tag = Self::parse_tag(record_content);
        let offset = Self::parse_offset(record_content);
        let length = Self::parse_length(record_content);
        Self {
            tag: tag,
            checksum: Self::parse_checksum(record_content),
            offset: offset,
            length: length,
            table_data: &file_content[offset..offset + length],
        }
    }

    fn parse_tag(content: &[u8]) -> [char; 4] {
        let tag = &content[TABLE_TAG_OFFSET..TABLE_TAG_OFFSET + TABLE_TAG_LENGTH];
        [
            tag[0] as char,
            tag[1] as char,
            tag[2] as char,
            tag[3] as char,
        ]
    }

    fn parse_checksum(content: &[u8]) -> u32 {
        BigEndian::read_u32(&content[TABLE_CHECKSUM_OFFSET..TABLE_CHECKSUM_OFFSET + 4])
    }

    fn parse_offset(content: &[u8]) -> usize {
        BigEndian::read_u32(&content[TABLE_OFFSET_OFFSET..TABLE_OFFSET_OFFSET + 4]) as usize
    }

    fn parse_length(content: &[u8]) -> usize {
        BigEndian::read_u32(&content[TABLE_LENGTH_OFFSET..TABLE_LENGTH_OFFSET + 4]) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header() {
        let mut content = vec![0x00u8; 47252];
        content[..12].clone_from_slice(&[
            0x00u8, 0x01, 0x00, 0x00, 0x00, 0x11, 0x01, 0x00, 0x00, 0x04, 0x00, 0x10,
        ]);
        assert_eq!(SfntFile::parse_num_tables(&content), 17);
        assert_eq!(SfntFile::parse_search_range(&content), 256);
        assert_eq!(SfntFile::parse_entry_selector(&content), 4);
        assert_eq!(SfntFile::parse_range_shift(&content), 16);
    }

    #[test]
    fn parse_table_records() {
        let mut content = vec![0x00u8; 47252];
        content[5] = 0x12;
        content[12..12 + 4].clone_from_slice(&[0x6Eu8, 0x61, 0x6D, 0x65]);
        content[12 + 32..12 + 32 + 4].clone_from_slice(&[0x67u8, 0x6C, 0x79, 0x66]);

        let table_records = SfntFile::parse_table_records(&content);

        assert_eq!(table_records.len(), 18);
        assert_eq!(table_records[0].tag, ['n', 'a', 'm', 'e']);
        assert_eq!(table_records[2].tag, ['g', 'l', 'y', 'f']);
    }

    #[test]
    fn parse_nth_table_record_first() {
        let mut content = vec![0x00u8; 47252];
        content[12..12 + 4].clone_from_slice(&[0x6Eu8, 0x61, 0x6D, 0x65]);
        content[16..16 + 4].clone_from_slice(&[0xFCu8, 0xFD, 0xFE, 0xFF]);

        let rec0 = SfntFile::parse_nth_table_record(&content, 0);
        assert_eq!(rec0.tag, ['n', 'a', 'm', 'e']);
        assert_eq!(rec0.checksum, 0xFCFDFEFF);
    }

    #[test]
    fn parse_nth_table_record_offset() {
        let mut content = vec![0x00u8; 47252];
        content[12 + 32..12 + 32 + 4].clone_from_slice(&[0x6Eu8, 0x61, 0x6D, 0x65]);

        let rec2 = SfntFile::parse_nth_table_record(&content, 2);
        assert_eq!(rec2.tag, ['n', 'a', 'm', 'e']);
    }

    #[test]
    fn deserialize_table_record() {
        let mut rec_content = vec![0x00u8; 16];
        rec_content[0..4].clone_from_slice(&[0x6Eu8, 0x61, 0x6D, 0x65]);
        rec_content[4..8].clone_from_slice(&[0xFCu8, 0xFD, 0xFE, 0xFF]);
        rec_content[8..12].clone_from_slice(&[0x00u8, 0x00, 0x00, 0x10]);
        rec_content[12..16].clone_from_slice(&[0x00u8, 0x00, 0x00, 0x04]);

        let mut file_content = vec![0x00u8; 20];
        file_content[0..16].clone_from_slice(&rec_content);
        file_content[16..20].clone_from_slice(&[0x01u8, 0x02, 0x03, 0x04]);

        let rec0 = TableRecord::deserialize(&rec_content, &file_content);
        assert_eq!(rec0.tag, ['n', 'a', 'm', 'e']);
        assert_eq!(rec0.checksum, 0xFCFDFEFF);
        assert_eq!(rec0.offset, 0x00000010);
        assert_eq!(rec0.length, 0x00000004);
        assert_eq!(rec0.table_data, &[0x01u8, 0x02, 0x03, 0x04]);
    }
}
