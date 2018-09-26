use super::encoding::Encoding;
use super::name::Name;
use super::platform::Platform;
use super::record::{NameRecord, ParseError};
use byteorder::{BigEndian, ByteOrder};

const U16_LENGTH: usize = 2;

/// The name table stores strings which represent various metadata in
/// the font (e.g., foundry name, font name, etc.).
#[derive(Debug)]
pub struct NameTable<'a> {
    /// NameTable comes in two formats, 0 and 1.
    /// The latter supports language-tag records.
    pub format: Format,
    /// The number of name records in the table.
    pub count: u16,
    /// The offset of the string storage region (from the start of the table).
    pub string_offset: usize,
    /// The parsed name records.
    pub name_records: Vec<NameRecord>,
    string_storage: &'a [u8],
}

impl<'a> NameTable<'a> {
    const FORMAT_OFFSET: usize = 0;
    const COUNT_OFFSET: usize = 2;
    const STRING_OFFSET_OFFSET: usize = 4;
    const NAME_RECORDS_OFFSET: usize = 6;
    const NAME_RECORD_LENGTH: usize = 12;

    /// Deserialize the name table from font file data.
    pub fn deserialize(table_data: &'a [u8]) -> Result<Self, ParseError> {
        let string_offset: usize = Self::parse_string_offset(table_data);
        Ok(NameTable {
            format: Self::parse_format(table_data)?,
            count: Self::parse_record_count(table_data),
            string_offset: string_offset,
            name_records: Self::parse_name_records(table_data)?,
            string_storage: Self::parse_string_storage(table_data, string_offset),
        })
    }

    pub fn read_string_value(
        &self,
        platform: Platform,
        encoding: Encoding,
        name: Name,
    ) -> Option<&'a [u8]> {
        let mut result = None;
        for record in &self.name_records {
            if record.platform == platform
                && record.encoding == encoding
                && record.name == Some(name.clone())
            {
                result = Some(
                    &self.string_storage
                        [record.string_offset..record.string_offset + record.string_length],
                );
            }
        }
        result
    }

    fn parse_string_storage<'b>(table_data: &'b [u8], offset: usize) -> &'b [u8] {
        let storage_length = table_data.len() - offset as usize;
        &table_data[offset..offset + storage_length]
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

    fn parse_string_offset(data: &[u8]) -> usize {
        BigEndian::read_u16(
            &data[Self::STRING_OFFSET_OFFSET..Self::STRING_OFFSET_OFFSET + U16_LENGTH],
        ) as usize
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

#[cfg(test)]
mod tests {
    use super::*;
    use opentype::name::encoding;

    const SAMPLE_TABLE: [u8; 32] = [
        0u8, 0, 0, 1, 0, 18, 0, 0, 0, 0, 0, 0, 0, 1, 0, 14, 0, 0, 0, 82, 0, 101, 0, 103, 0, 117, 0,
        108, 0, 97, 0, 114,
    ];
    const SAMPLE_HEADER: [u8; 6] = [0u8, 0, 0, 26, 1, 62];

    #[test]
    fn parse_format_0() {
        let mut data = vec![0u8; 1062];
        data[..6].clone_from_slice(&SAMPLE_HEADER);

        assert_eq!(NameTable::parse_format(&data), Ok(Format::Format0));
    }

    #[test]
    fn parse_record_count() {
        let mut data = vec![0u8; 1062];
        data[..6].clone_from_slice(&SAMPLE_HEADER);

        assert_eq!(NameTable::parse_record_count(&data), 26);
    }

    #[test]
    fn parse_string_offset() {
        let mut data = vec![0u8; 1062];
        data[..6].clone_from_slice(&SAMPLE_HEADER);

        assert_eq!(NameTable::parse_string_offset(&data), 318);
    }

    #[test]
    fn read_string_value() {
        let table = NameTable::deserialize(&SAMPLE_TABLE).unwrap();
        let result = table.read_string_value(
            Platform::Unicode,
            Encoding::Unicode {
                encoding: encoding::UnicodeEncoding::Unicode1,
            },
            Name::FontFamilyName,
        );

        const EXPECTED: [u8; 14] = [0u8, 82, 0, 101, 0, 103, 0, 117, 0, 108, 0, 97, 0, 114];
        assert_eq!(result, Some(&EXPECTED[..]));
    }
}
