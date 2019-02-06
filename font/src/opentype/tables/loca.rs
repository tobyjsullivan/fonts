use super::head::IndexToLocFormat;
use opentype::types::{DataType, Offset, Offset16, Offset32};

#[derive(Debug)]
pub struct LocaTable {
    pub num_glyphs: u16,
    locations: Vec<Location>,
}

impl LocaTable {
    pub fn parse(
        table_data: &[u8],
        version: IndexToLocFormat,
        num_glyphs: u16,
        glyf_len: usize,
    ) -> Self {
        let mut locations = Vec::new();
        for i in 0..num_glyphs as usize {
            let location = Self::location(table_data, version, num_glyphs, glyf_len, i);
            locations.push(location);
        }

        Self {
            num_glyphs,
            locations,
        }
    }

    pub fn index(&self, idx: usize) -> Location {
        if idx as u16 >= self.num_glyphs {
            panic!("Index out of range.");
        }

        *self.locations.get(idx).unwrap()
    }

    pub fn locations(&self) -> impl Iterator<Item = &Location> {
        self.locations.iter()
    }

    fn location(
        table_data: &[u8],
        version: IndexToLocFormat,
        num_glyphs: u16,
        glyf_len: usize,
        idx: usize,
    ) -> Location {
        let offset = Self::value_at(table_data, version, idx);

        let next = match idx + 1 == num_glyphs as usize {
            true => glyf_len,
            false => Self::value_at(table_data, version, idx + 1),
        };

        Location {
            offset,
            length: next - offset,
        }
    }

    fn value_at(table_data: &[u8], version: IndexToLocFormat, idx: usize) -> Offset {
        match version {
            IndexToLocFormat::ShortOffset => Offset16::extract(table_data, idx * 2usize) * 2usize,
            IndexToLocFormat::LongOffset => Offset32::extract(table_data, idx * 4usize),
            _ => {
                panic!("Unknown loca format {:?}", version);
            }
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Location {
    pub(crate) offset: usize,
    pub(crate) length: usize,
}
