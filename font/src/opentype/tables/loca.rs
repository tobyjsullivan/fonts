use super::head::IndexToLocFormat;
use opentype::types::{DataType, Offset, Offset16, Offset32};

#[derive(Debug)]
pub struct LocaTable<'a> {
    table_data: &'a [u8],
    version: IndexToLocFormat,
    pub num_glyphs: u16,
}

impl<'a> LocaTable<'a> {
    pub fn parse(table_data: &'a [u8], version: IndexToLocFormat, num_glyphs: u16) -> Self {
        Self {
            table_data,
            version,
            num_glyphs,
        }
    }

    pub fn index(&self, idx: usize) -> Location {
        if idx as u16 >= self.num_glyphs {
            panic!("Index out of range.");
        }

        let offset = self.value_at(idx);
        let next = self.value_at(idx + 1);

        Location {
            offset,
            length: next - offset,
        }
    }

    fn value_at(&self, idx: usize) -> Offset {
        if idx as u16 > self.num_glyphs {
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

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Location {
    pub(crate) offset: usize,
    pub(crate) length: usize,
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

        assert_eq!(
            table.index(1),
            Location {
                offset: 128usize,
                length: 200
            }
        );
    }

    #[test]
    fn index_long() {
        const EXAMPLE_TABLE_DATA: [u8; 12] = [0u8, 0, 0, 0, 0, 0, 0, 248, 0, 0, 1, 37];
        let table = LocaTable {
            table_data: &EXAMPLE_TABLE_DATA,
            version: IndexToLocFormat::LongOffset,
            num_glyphs: 4,
        };

        assert_eq!(
            table.index(1),
            Location {
                offset: 248usize,
                length: 45
            }
        );
    }
}
