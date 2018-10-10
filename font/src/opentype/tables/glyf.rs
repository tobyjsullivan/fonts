use opentype::types::{DataType, I16};

#[derive(Debug)]
pub struct GlyfTable<'a> {
    table_data: &'a [u8],
}

impl<'a> GlyfTable<'a> {
    pub fn parse(table_data: &'a [u8]) -> Self {
        Self {
            table_data,
        }
    }

    pub fn read_glyph(&self, offset: usize, len: usize) -> Glyph {
        Glyph::deserialize(&self.table_data[offset..offset+len])
    }
}

pub struct Glyph {
    num_contours: i16,
    min_x: i16,
    min_y: i16,
    max_x: i16,
    max_y: i16,
}

impl Glyph {
    fn deserialize(glyph_data: &[u8]) -> Self {
        const OFFSET_NUM_CONTOURS: usize = 0;
        const OFFSET_MIN_X: usize = 2;
        const OFFSET_MIN_Y: usize = 4;
        const OFFSET_MAX_X: usize = 6;
        const OFFSET_MAX_Y: usize = 8;

        Self {
            num_contours: I16::extract(glyph_data, OFFSET_NUM_CONTOURS),
            min_x: I16::extract(glyph_data, OFFSET_MIN_X),
            min_y: I16::extract(glyph_data, OFFSET_MIN_Y),
            max_x: I16::extract(glyph_data, OFFSET_MAX_X),
            max_y: I16::extract(glyph_data, OFFSET_MAX_Y),
        }
    }
}