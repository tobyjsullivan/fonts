use super::head::IndexToLocFormat;
use opentype::types::{DataType, Offset, Offset16, Offset32};

#[derive(Debug)]
pub struct LocaTable {
    pub num_glyphs: u16,
    offsets: Vec<usize>,
    glyf_len: usize,
}

impl LocaTable {
    pub fn parse(
        table_data: &[u8],
        version: IndexToLocFormat,
        num_glyphs: u16,
        glyf_len: usize,
    ) -> Self {
        let mut offsets = Vec::new();
        for i in 0..num_glyphs as usize {
            offsets.push(Self::offset(table_data, version, i));
        }

        Self {
            num_glyphs,
            offsets,
            glyf_len,
        }
    }

    pub fn index(&self, idx: usize) -> Option<Location> {
        if idx as u16 >= self.num_glyphs {
            panic!("Index out of range.");
        }

        calc_location(&self.offsets, self.glyf_len, idx)
    }

    pub fn locations(&self) -> impl Iterator<Item = Location> {
        LocationIter::new(self.offsets.clone(), self.glyf_len)
    }

    fn offset(table_data: &[u8], version: IndexToLocFormat, idx: usize) -> usize {
        Self::value_at(table_data, version, idx)
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

struct LocationIter {
    cursor: usize,
    offsets: Vec<usize>,
    glyf_len: usize,
}

impl LocationIter {
    fn new(offsets: Vec<usize>, glyf_len: usize) -> Self {
        Self {
            cursor: 0,
            offsets,
            glyf_len,
        }
    }
}

impl Iterator for LocationIter {
    type Item = Location;

    fn next(&mut self) -> Option<Location> {
        let idx = self.cursor;
        self.cursor += 1;
        calc_location(&self.offsets, self.glyf_len, idx)
    }
}

fn calc_location(offsets: &Vec<usize>, glyf_len: usize, idx: usize) -> Option<Location> {
    match (offsets.get(idx), offsets.get(idx + 1)) {
        (Some(offset), Some(next)) => Some(Location {
            offset: *offset,
            length: next - offset,
        }),
        (Some(offset), None) => Some(Location {
            offset: *offset,
            length: glyf_len - offset,
        }),
        (None, _) => None,
    }
}
