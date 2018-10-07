use super::head::IndexToLocFormat;
use opentype::types::{DataType, Offset, Offset16, Offset32};

#[derive(Debug)]
pub struct LocaTable {
    offsets: Vec<Offset>,
}

impl LocaTable {
    pub fn parse(table_data: &[u8], version: IndexToLocFormat, num_glyphs: u16) -> Self {
        let mut offsets = Vec::new();
        let mut head = 0usize;

        for _i in 0..num_glyphs {
            let offset = match version {
                IndexToLocFormat::ShortOffset => {
                    let res = Offset16::extract(table_data, head) * 2usize;
                    head += 2usize;
                    res
                }
                IndexToLocFormat::LongOffset => {
                    let res = Offset32::extract(table_data, head);
                    head += 4usize;
                    res
                }
                _ => {
                    panic!("Unknown loca format {:?}", version);
                }
            };
            offsets.push(offset);
        }

        Self { offsets }
    }
}
