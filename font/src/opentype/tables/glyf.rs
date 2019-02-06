use opentype::tables::loca::LocaTable;
use opentype::types::{DataType, I16, U16, U8};

#[derive(Debug)]
pub struct GlyfTable {
    glyphs: Vec<Option<Glyph>>,
}

impl GlyfTable {
    pub fn parse(table_data: &[u8], loca_table: &LocaTable) -> Self {
        let mut glyphs = Vec::new();
        for location in loca_table.locations() {
            let len = location.length;
            if len == 0 {
                glyphs.push(None);
                continue;
            }

            let offset = location.offset;
            let glyph = Glyph::deserialize(&table_data[offset..offset + len]);
            glyphs.push(Some(glyph));
        }

        Self { glyphs }
    }

    pub fn read_glyph(&self, loca_idx: usize) -> Option<Glyph> {
        self.glyphs.get(loca_idx).map(|o| o.clone()).unwrap_or(None)
    }
}

#[derive(Clone, Debug)]
pub struct Glyph {
    num_contours: i16,
    min_x: i16,
    min_y: i16,
    max_x: i16,
    max_y: i16,
    simple_glyph: Option<SimpleGlyphTable>,
    compound_glyph: Option<CompoundGlyphTable>,
}

impl Glyph {
    const OFFSET_NUM_CONTOURS: usize = 0;
    const OFFSET_MIN_X: usize = 2;
    const OFFSET_MIN_Y: usize = 4;
    const OFFSET_MAX_X: usize = 6;
    const OFFSET_MAX_Y: usize = 8;
    const OFFSET_TABLE_DATA: usize = 10;

    fn deserialize(glyph_data: &[u8]) -> Self {
        let num_contours = I16::extract(glyph_data, Self::OFFSET_NUM_CONTOURS);

        let mut simple_glyph = None;
        let mut compound_glyph = None;

        if num_contours >= 0 {
            simple_glyph = Some(SimpleGlyphTable::deserialize(
                &glyph_data[Self::OFFSET_TABLE_DATA..],
                num_contours,
            ));
        } else {
            // TODO: Deserialize compund glyph tables.
            compound_glyph = Some(CompoundGlyphTable {});
        }

        Self {
            num_contours,
            min_x: I16::extract(glyph_data, Self::OFFSET_MIN_X),
            min_y: I16::extract(glyph_data, Self::OFFSET_MIN_Y),
            max_x: I16::extract(glyph_data, Self::OFFSET_MAX_X),
            max_y: I16::extract(glyph_data, Self::OFFSET_MAX_Y),
            simple_glyph,
            compound_glyph,
        }
    }
}

#[derive(Clone, Debug)]
struct SimpleGlyphTable {
    end_points_of_contours: Vec<usize>,
    instruction_length: usize,
    instructions: Vec<u8>,
    flags: Vec<u8>,
    x_coordinates: Vec<i16>,
    y_coordinates: Vec<i16>,
}

impl<'a> SimpleGlyphTable {
    const MASK_X_SHORT_VECTOR: u8 = 0b0000_0010;
    const MASK_Y_SHORT_VECTOR: u8 = 0b0000_0100;
    const MASK_REPEAT_FLAG: u8 = 0b0000_1000;
    const MASK_X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR: u8 = 0b0001_0000;
    const MASK_Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR: u8 = 0b0010_0000;

    fn deserialize(table_data: &'a [u8], num_contours: i16) -> Self {
        let mut end_points_of_contours = vec![];
        let mut cursor = 0usize;
        let mut end_point = 0;

        for _ in 0..num_contours {
            end_point = U16::extract(table_data, cursor);
            end_points_of_contours.push(end_point as usize);
            cursor += 2;
        }

        let instruction_length = U16::extract(table_data, cursor) as usize;
        cursor += 2;

        let offset_instructions = cursor;
        cursor += instruction_length;

        let flags = Self::deserialize_flags(table_data, &mut cursor, end_point + 1);
        let x_coordinates =
            Self::deserialize_x_coordinates(table_data, &mut cursor, end_point + 1, &flags);
        let y_coordinates =
            Self::deserialize_y_coordinates(table_data, &mut cursor, end_point + 1, &flags);

        Self {
            end_points_of_contours,
            instruction_length,
            instructions: table_data[offset_instructions..offset_instructions + instruction_length]
                .to_vec(),
            flags,
            x_coordinates,
            y_coordinates,
        }
    }

    fn deserialize_flags<'b>(table_data: &[u8], cursor: &'b mut usize, num_points: u16) -> Vec<u8> {
        let mut flags = vec![];
        let mut flag_idx = 0;
        while flag_idx < num_points {
            let flag = U8::extract(table_data, *cursor);
            *cursor += 1;

            let repetitions = if flag & Self::MASK_REPEAT_FLAG == 0 {
                0
            } else {
                let r = U8::extract(table_data, *cursor);
                *cursor += 1;
                r
            };

            for _ in 0..repetitions + 1 {
                flags.push(flag);
                flag_idx += 1;
            }
        }

        flags
    }

    fn deserialize_x_coordinates(
        table_data: &[u8],
        cursor: &mut usize,
        num_points: u16,
        flags: &Vec<u8>,
    ) -> Vec<i16> {
        let mut x_coordinates: Vec<i16> = vec![];
        let mut x_coord_idx = 0;
        let mut previous_x = 0i16;
        while x_coord_idx < num_points as usize {
            let flag = flags[x_coord_idx];

            let parse_flags = (
                flag & Self::MASK_X_SHORT_VECTOR != 0,
                flag & Self::MASK_X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR != 0,
            );
            let x_coordinate = match parse_flags {
                (true, true) => {
                    let parsed = U8::extract(table_data, *cursor);
                    *cursor += 1;
                    parsed as i16
                }
                (true, false) => {
                    let parsed = U8::extract(table_data, *cursor);
                    *cursor += 1;
                    0 - parsed as i16
                }
                (false, true) => previous_x,
                (false, false) => {
                    let parsed = I16::extract(table_data, *cursor);
                    *cursor += 2;
                    parsed
                }
            };

            x_coordinates.push(x_coordinate);
            x_coord_idx += 1;
            previous_x = x_coordinate;
        }

        x_coordinates
    }
    fn deserialize_y_coordinates(
        table_data: &[u8],
        cursor: &mut usize,
        num_points: u16,
        flags: &Vec<u8>,
    ) -> Vec<i16> {
        let mut y_coordinates: Vec<i16> = vec![];
        let mut y_coord_idx = 0;
        let mut previous_y = 0i16;
        while y_coord_idx < num_points as usize {
            let flag = flags[y_coord_idx];

            let parse_flags = (
                flag & Self::MASK_Y_SHORT_VECTOR != 0,
                flag & Self::MASK_Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR != 0,
            );
            let y_coordinate = match parse_flags {
                (true, true) => {
                    let parsed = U8::extract(table_data, *cursor);
                    *cursor += 1;
                    parsed as i16
                }
                (true, false) => {
                    let parsed = U8::extract(table_data, *cursor);
                    *cursor += 1;
                    0 - parsed as i16
                }
                (false, true) => previous_y,
                (false, false) => {
                    let parsed = I16::extract(table_data, *cursor);
                    *cursor += 2;
                    parsed
                }
            };

            y_coordinates.push(y_coordinate);
            y_coord_idx += 1;
            previous_y = y_coordinate;
        }

        y_coordinates
    }
}

#[derive(Clone, Debug)]
struct CompoundGlyphTable {}
