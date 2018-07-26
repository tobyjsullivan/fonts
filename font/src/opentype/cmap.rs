/// The cmap table is a map from character codes to glyphs or, more accurately, indexes in the
/// glyf table.
use byteorder::{ByteOrder, BigEndian};

const U16_LENGTH: usize = 2;
const VERSION_OFFSET: usize = 0;
const VERSION_LENGTH: usize = 2;
const NUM_TABLES_OFFSET: usize = 2;
const NUM_TABLES_LENGTH: usize = 2;

#[derive(Debug)]
pub struct CmapTable {
  table_version: u16,
  num_tables: u16,
}

impl CmapTable {
  pub fn deserialize(data: &[u8]) -> Self {
    CmapTable {
      table_version: CmapTable::parse_version(data),
      num_tables: CmapTable::parse_num_tables(data),
    }
  }

  fn parse_version(data: &[u8]) -> u16 {
    BigEndian::read_u16(&data[VERSION_OFFSET..VERSION_OFFSET+U16_LENGTH])
  }

  fn parse_num_tables(data: &[u8]) -> u16 {
    BigEndian::read_u16(&data[NUM_TABLES_OFFSET..NUM_TABLES_OFFSET+U16_LENGTH])
  }
}

#[cfg(test)]
mod cmap_tests {
    use super::*;

    #[test]
    fn parse_version() {
        let mut content = vec![0x00u8; 100];
        content[..2].clone_from_slice(&[0x00u8, 0x01]);
        assert_eq!(CmapTable::parse_version(&content), 1u16);
    }

    #[test]
    fn parse_num_tables() {
        let mut content = vec![0x00u8; 100];
        content[2..4].clone_from_slice(&[0x00u8, 0x10]);
        assert_eq!(CmapTable::parse_num_tables(&content), 16u16);
    }
}
