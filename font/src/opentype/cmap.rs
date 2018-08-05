/// The cmap table is a map from character codes to glyphs or, more accurately, indexes in the
/// glyf table.
use byteorder::{ByteOrder, BigEndian};

const U16_LENGTH: usize = 2;
const VERSION_OFFSET: usize = 0;
const NUM_TABLES_OFFSET: usize = 2;

#[derive(Debug)]
pub struct CmapTable {
  table_version: Version,
  num_tables: u16,
}

impl CmapTable {
  pub fn deserialize(data: &[u8]) -> Result<Self, ParseError> {
    Ok(CmapTable {
      table_version: CmapTable::parse_version(data)?,
      num_tables: CmapTable::parse_num_tables(data),
    })
  }

  fn parse_version(data: &[u8]) -> Result<Version, ParseError> {
    match BigEndian::read_u16(&data[VERSION_OFFSET..VERSION_OFFSET+U16_LENGTH]) {
      0 => Ok(Version::Version0),
      _ => Err(ParseError::UnknownVersion),
    }
  }

  fn parse_num_tables(data: &[u8]) -> u16 {
    BigEndian::read_u16(&data[NUM_TABLES_OFFSET..NUM_TABLES_OFFSET+U16_LENGTH])
  }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
  UnknownVersion
}

#[derive(Debug, PartialEq)]
pub enum Version {
  // V0 is, in fact, the only documented version so far.
  // https://docs.microsoft.com/en-us/typography/opentype/spec/cmap
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
        assert_eq!(CmapTable::parse_version(&content), Err(ParseError::UnknownVersion));
    }

    #[test]
    fn parse_num_tables() {
        let mut content = vec![0x00u8; 100];
        content[2..4].clone_from_slice(&[0x00u8, 0x10]);
        assert_eq!(CmapTable::parse_num_tables(&content), 16u16);
    }
}
