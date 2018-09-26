use super::encoding::Encoding;
use super::name::Name;
use super::platform::Platform;
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
    pub string_offset: usize,
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
    pub fn deserialize(table_data: &[u8]) -> Result<Self, ParseError> {
        println!("name table data: {:?}", table_data);
        Ok(NameTable {
            format: Self::parse_format(table_data)?,
            count: Self::parse_record_count(table_data),
            string_offset: Self::parse_string_offset(table_data),
            name_records: Self::parse_name_records(table_data)?,
        })
    }

    pub fn parse_value<'a>(
        &self,
        table_data: &'a [u8],
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
                let storage = self.string_storage(table_data);
                result = Some(
                    &storage[record.string_offset..record.string_offset + record.string_length],
                );
            }
        }
        result
    }

    fn string_storage<'a>(&self, table_data: &'a [u8]) -> &'a [u8] {
        let storage_length = table_data.len() - self.string_offset as usize;
        &table_data[self.string_offset..self.string_offset + storage_length]
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
    use super::super::encoding;
    use super::*;

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
    fn parse_value() {
        let table = NameTable::deserialize(&SAMPLE_TABLE).unwrap();
        let result = table.parse_value(
            &SAMPLE_TABLE,
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
