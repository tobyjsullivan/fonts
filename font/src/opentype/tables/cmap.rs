//! The cmap table is a map from character codes to glyphs or, more accurately, indexes in the
//! glyf table.

use opentype::encoding::Encoding;
use opentype::platform::Platform;
use opentype::types::{DataType, Offset32, U16};

#[derive(Debug)]
pub struct CmapTable {
    table_version: Version,
    num_tables: u16,
    encoding_records: Vec<EncodingRecord>,
    subtables: Vec<Subtable>,
}

impl CmapTable {
    const VERSION_OFFSET: usize = 0;
    const NUM_TABLES_OFFSET: usize = 2;
    const ENCODING_RECORDS_OFFSET: usize = 4;
    const ENCODING_RECORD_LENGTH: usize = 8;

    pub fn deserialize(data: &[u8]) -> Result<Self, ParseError> {
        let num_tables = U16::extract(data, Self::NUM_TABLES_OFFSET);
        let encoding_records = Self::parse_encoding_records(data, num_tables)?;
        let subtables = Self::parse_subtables(data, &encoding_records)?;

        Ok(CmapTable {
            table_version: Self::parse_version(data)?,
            num_tables,
            encoding_records,
            subtables,
        })
    }

    fn parse_version(data: &[u8]) -> Result<Version, ParseError> {
        match U16::extract(data, Self::VERSION_OFFSET) {
            0 => Ok(Version::Version0),
            _ => Err(ParseError::UnknownVersion),
        }
    }

    fn parse_encoding_records(
        data: &[u8],
        num_tables: u16,
    ) -> Result<Vec<EncodingRecord>, ParseError> {
        let mut records: Vec<EncodingRecord> = vec![];

        let mut offset = Self::ENCODING_RECORDS_OFFSET;
        for i in 0..num_tables {
            records.push(EncodingRecord::parse(
                &data[offset..offset + Self::ENCODING_RECORD_LENGTH],
            )?);
            offset += Self::ENCODING_RECORD_LENGTH;
        }

        Ok(records)
    }

    fn parse_subtables(
        data: &[u8],
        encoding_records: &Vec<EncodingRecord>,
    ) -> Result<Vec<Subtable>, ParseError> {
        let mut subtables: Vec<Subtable> = vec![];

        for r in encoding_records {
            subtables.push(Subtable::parse(data, r.offset)?);
        }

        Ok(subtables)
    }
}

#[derive(Debug)]
struct EncodingRecord {
    platform: Platform,
    encoding: Encoding,
    offset: usize,
}

impl EncodingRecord {
    const PLATFORM_OFFSET: usize = 0;
    const ENCODING_OFFSET: usize = 2;
    const SUBTABLE_OFFSET_OFFSET: usize = 4;

    fn parse(record_data: &[u8]) -> Result<Self, ParseError> {
        let platform = match Platform::lookup(U16::extract(record_data, Self::PLATFORM_OFFSET)) {
            None => return Err(ParseError::UnknownPlatform),
            Some(p) => p,
        };

        let encoding =
            match Encoding::lookup(platform, U16::extract(record_data, Self::ENCODING_OFFSET)) {
                None => return Err(ParseError::UnknownEncoding),
                Some(e) => e,
            };

        Ok(Self {
            platform,
            encoding,
            offset: Offset32::extract(record_data, Self::SUBTABLE_OFFSET_OFFSET),
        })
    }
}

#[derive(Debug)]
enum Subtable {
    /// Format 0: Byte encoding table
    Format0 {},
    /// Format 2: High-byte mapping through table
    Format2 {},
    /// Format 4: Segment mapping to delta values
    Format4 {},
    /// Format 6: Trimmed table mapping
    Format6 {},
    /// Format 8: mixed 16-bit and 32-bit coverage
    Format8 {},
    /// Format 10: Trimmed array
    Format10 {},
    /// Format 12: Segmented coverage
    Format12 {},
    /// Format 13: Many-to-one range mappings
    Format13 {},
    /// Format 14: Unicode Variation Sequences
    Format14 {},
}

impl Subtable {
    // This parser needs to take the entire cmap table because we don't actually know the length of the subtable until we detect the format.
    fn parse(cmap_data: &[u8], subtable_offset: usize) -> Result<Self, ParseError> {
        match U16::extract(cmap_data, subtable_offset) {
            0 => Ok(Subtable::Format0 {}),
            2 => Ok(Subtable::Format2 {}),
            4 => Ok(Subtable::Format4 {}),
            6 => Ok(Subtable::Format6 {}),
            8 => Ok(Subtable::Format8 {}),
            10 => Ok(Subtable::Format10 {}),
            12 => Ok(Subtable::Format12 {}),
            13 => Ok(Subtable::Format13 {}),
            14 => Ok(Subtable::Format14 {}),
            _ => Err(ParseError::UnknownSubtableFormat),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownVersion,
    UnknownPlatform,
    UnknownEncoding,
    UnknownSubtableFormat,
}

#[derive(Debug, PartialEq)]
pub enum Version {
    /// V0 is, in fact, the only documented version so far.
    /// https://docs.microsoft.com/en-us/typography/opentype/spec/cmap
    Version0,
}

#[cfg(test)]
mod cmap_tests {
    use super::*;

    #[test]
    fn parse_version_0() {
        let mut content = vec![0x00u8; 100];
        content[..2].clone_from_slice(&[0x00u8, 0x00]);
        assert_eq!(CmapTable::parse_version(&content), Ok(Version::Version0));
    }

    #[test]
    fn parse_version_unknown() {
        let mut content = vec![0x00u8; 100];
        content[..2].clone_from_slice(&[0x00u8, 0x01]);
        assert_eq!(
            CmapTable::parse_version(&content),
            Err(ParseError::UnknownVersion)
        );
    }
}
