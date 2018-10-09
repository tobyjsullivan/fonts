use super::head::IndexToLocFormat;
use opentype::types::{DataType, Offset, Offset16, Offset32};

use std::ops::Index;

#[derive(Debug)]
pub struct LocaTable<'a> {
    table_data: &'a [u8],
    version: IndexToLocFormat,
    num_glyphs: u16,
}

impl<'a> LocaTable<'a> {
    pub fn parse(table_data: &'a [u8], version: IndexToLocFormat, num_glyphs: u16) -> Self {
        Self {
            table_data,
            version,
            num_glyphs,
        }
    }

    pub fn index(&self, idx: usize) -> Offset {
        if idx as u16 >= self.num_glyphs {
            panic!("Index out of range.");
        }

        match self.version {
            IndexToLocFormat::ShortOffset => {
                Offset16::extract(self.table_data, idx * 2usize) * 2usize
            }
            IndexToLocFormat::LongOffset => Offset32::extract(self.table_data, idx * 4usize),
            _ => {
                panic!("Unknown loca format {:?}", self.version);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn index_short() {
        const EXAMPLE_TABLE_DATA: [u8; 8] = [0u8, 0, 0, 64, 0, 164, 0, 248];
        let table = LocaTable {
            table_data: &EXAMPLE_TABLE_DATA,
            version: IndexToLocFormat::ShortOffset,
            num_glyphs: 4,
        };

        assert_eq!(table.index(1), 128usize);
    }

    #[test]
    fn index_long() {
        const EXAMPLE_TABLE_DATA: [u8; 8] = [0u8, 0, 0, 0, 0, 0, 0, 248];
        let table = LocaTable {
            table_data: &EXAMPLE_TABLE_DATA,
            version: IndexToLocFormat::LongOffset,
            num_glyphs: 4,
        };

        assert_eq!(table.index(1), 248usize);
    }
}
