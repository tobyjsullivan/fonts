use super::name::Name;
use super::record::{NameRecord, ParseError};
use opentype::encoding::Encoding;
use opentype::platform::Platform;
use opentype::types::{DataType, Offset16, U16};

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
        let string_offset = Offset16::extract(table_data, Self::STRING_OFFSET_OFFSET);
        let string_storage = Self::parse_string_storage(table_data, string_offset);
        let name_records = Self::parse_name_records(table_data, &string_storage)?;

        Ok(NameTable {
            format: Self::parse_format(table_data)?,
            count: U16::extract(table_data, Self::COUNT_OFFSET),
            string_offset: string_offset,
            name_records: name_records,
        })
    }

    /// Read out the specified string value if it exists in the name table.
    pub fn read_string_value(
        &self,
        platform: Platform,
        encoding: Encoding,
        name: Name,
    ) -> Option<&[u8]> {
        let mut result = None;
        for record in &self.name_records {
            if record.platform == platform
                && record.encoding == encoding
                && record.name == Some(name.clone())
            {
                result = Some(&record.data[..]);
            }
        }
        result
    }

    pub fn available_strings(&self) -> Vec<(Name, Encoding, Vec<u8>)> {
        let mut names = Vec::new();

        for record in &self.name_records {
            if let Some(name) = record.name {
                names.push((name, record.encoding, record.data.clone()));
            }
        }
        names
    }

    pub fn find_strings(&self, name: Name) -> Vec<(Encoding, &[u8])> {
        let mut result = vec![];

        for record in &self.name_records {
            if record.name == Some(name) {
                result.push((record.encoding, &record.data[..]));
            }
        }

        result
    }

    fn parse_string_storage(table_data: &[u8], offset: usize) -> &[u8] {
        let storage_length = table_data.len() - offset as usize;
        &table_data[offset..offset + storage_length]
    }

    fn parse_format(data: &[u8]) -> Result<Format, ParseError> {
        let value = U16::extract(data, Self::FORMAT_OFFSET);
        match value {
            0 => Ok(Format::Format0),
            1 => Ok(Format::Format1),
            _ => Err(ParseError::UnknownFormat),
        }
    }

    fn parse_name_records(
        data: &[u8],
        string_storage: &[u8],
    ) -> Result<Vec<NameRecord>, ParseError> {
        let mut records = vec![];
        let num_records = U16::extract(data, Self::COUNT_OFFSET);
        for n in 0..num_records {
            let offset = Self::NAME_RECORDS_OFFSET + n as usize * Self::NAME_RECORD_LENGTH;
            let record_data = &data[offset..offset + Self::NAME_RECORD_LENGTH];

            records.push(NameRecord::deserialize(record_data, string_storage)?);
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

    const SAMPLE_TABLE: [u8; 32] = [
        0u8, 0, 0, 1, 0, 18, 0, 0, 0, 0, 0, 0, 0, 1, 0, 14, 0, 0, 0, 82, 0, 101, 0, 103, 0, 117, 0,
        108, 0, 97, 0, 114,
    ];

    #[test]
    fn read_string_value() {
        let table = NameTable::deserialize(&SAMPLE_TABLE).unwrap();
        let result =
            table.read_string_value(Platform::Unicode, Encoding::Unicode1, Name::FontFamilyName);

        const EXPECTED: [u8; 14] = [0u8, 82, 0, 101, 0, 103, 0, 117, 0, 108, 0, 97, 0, 114];
        assert_eq!(result, Some(&EXPECTED[..]));
    }
}
