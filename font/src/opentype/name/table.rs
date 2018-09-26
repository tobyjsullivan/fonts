use super::record::{NameRecord, ParseError};
use byteorder::{BigEndian, ByteOrder};

const U16_LENGTH: usize = 2;

/// The name table stores strings which represent various metadata in
/// the font (e.g., foundry name, font name, etc.).
#[derive(Debug)]
pub struct NameTable {
    /// NameTable comes in two formats, 0 and 1.
    /// The latter supports language-tag records.
    pub format: Format,
    /// The number of name records in the table.
    pub count: u16,
    /// The offset of the string storage region (from the start of the table).
    pub string_offset: u16,
    /// The parsed name records.
    pub name_records: Vec<NameRecord>,
}

impl NameTable {
    const FORMAT_OFFSET: usize = 0;
    const COUNT_OFFSET: usize = 2;
    const STRING_OFFSET_OFFSET: usize = 4;
    const NAME_RECORDS_OFFSET: usize = 6;
    const NAME_RECORD_LENGTH: usize = 12;

    /// Deserialize the name table from font file data.
    pub fn deserialize(data: &[u8]) -> Result<Self, ParseError> {
        Ok(NameTable {
            format: Self::parse_format(data)?,
            count: Self::parse_record_count(data),
            string_offset: Self::parse_string_offset(data),
            name_records: Self::parse_name_records(data)?,
        })
    }

    fn parse_format(data: &[u8]) -> Result<Format, ParseError> {
        match BigEndian::read_u16(&data[Self::FORMAT_OFFSET..Self::FORMAT_OFFSET + U16_LENGTH]) {
            0 => Ok(Format::Format0),
            1 => Ok(Format::Format1),
            _ => Err(ParseError::UnknownFormat),
        }
    }

    fn parse_record_count(data: &[u8]) -> u16 {
        BigEndian::read_u16(&data[Self::COUNT_OFFSET..Self::COUNT_OFFSET + U16_LENGTH])
    }

    fn parse_string_offset(data: &[u8]) -> u16 {
        BigEndian::read_u16(
            &data[Self::STRING_OFFSET_OFFSET..Self::STRING_OFFSET_OFFSET + U16_LENGTH],
        )
    }

    fn parse_name_records(data: &[u8]) -> Result<Vec<NameRecord>, ParseError> {
        let mut records = vec![];
        let num_records = Self::parse_record_count(data);
        for n in 0..num_records {
            let offset = Self::NAME_RECORDS_OFFSET + n as usize * Self::NAME_RECORD_LENGTH;
            let record_data = &data[offset..offset + Self::NAME_RECORD_LENGTH];

            records.push(NameRecord::deserialize(record_data)?);
        }

        Ok(records)
    }
}

/// Supported formats for Name Tables.
#[derive(Debug, PartialEq)]
pub enum Format {
    Format0,
    Format1,
}
