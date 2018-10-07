use opentype::types::{DataType, Datetime, Fixed, I16, U16, U32};

const OFFSET_MAJOR_VERSION: usize = 0;
const OFFSET_MINOR_VERSION: usize = 2;
const OFFSET_FONT_REVISION: usize = 4;
const OFFSET_CHECKSUM_ADJ: usize = 8;
const OFFSET_MAGIC_NUMBER: usize = 12;
const OFFSET_FLAGS: usize = 16;
const OFFSET_UNITS_PER_EM: usize = 18;
const OFFSET_CREATED: usize = 20;
const OFFSET_MODIFIED: usize = 28;
const OFFSET_X_MIN: usize = 36;
const OFFSET_Y_MIN: usize = 38;
const OFFSET_X_MAX: usize = 40;
const OFFSET_Y_MAX: usize = 42;
const OFFSET_LOWEST_PPEM: usize = 44;
const OFFSET_MAC_STYLE: usize = 46;
const OFFSET_FONT_DIR_HINT: usize = 48;
const OFFSET_INDEX_TO_LOC_FMT: usize = 50;
const OFFSET_GLYPH_DATA_FMT: usize = 52;

#[derive(Debug)]
pub struct HeadTable {
    major_version: u16,
    minor_version: u16,
    font_revision: Fixed,
    checksum_adjustment: u32,
    magic_number: u32,
    flags: u16,
    units_per_em: u16,
    created: Datetime,
    modified: Datetime,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    /// The lowest recommended size in pixels.
    lowest_rec_ppem: u16,
    mac_style: u16,
    font_dir_hint: FontDirectionHint,
    pub index_to_loc_fmt: IndexToLocFormat,
    glyph_data_fmt: GlyphDataFormat,
}

impl HeadTable {
    pub fn parse(table_data: &[u8]) -> Self {
        Self {
            major_version: U16::extract(table_data, OFFSET_MAJOR_VERSION),
            minor_version: U16::extract(table_data, OFFSET_MINOR_VERSION),
            font_revision: Fixed::extract(table_data, OFFSET_FONT_REVISION),
            checksum_adjustment: U32::extract(table_data, OFFSET_CHECKSUM_ADJ),
            magic_number: U32::extract(table_data, OFFSET_MAGIC_NUMBER),
            flags: U16::extract(table_data, OFFSET_FLAGS),
            units_per_em: U16::extract(table_data, OFFSET_UNITS_PER_EM),
            created: Datetime::extract(table_data, OFFSET_CREATED),
            modified: Datetime::extract(table_data, OFFSET_MODIFIED),
            x_min: I16::extract(table_data, OFFSET_X_MIN),
            y_min: I16::extract(table_data, OFFSET_Y_MIN),
            x_max: I16::extract(table_data, OFFSET_X_MAX),
            y_max: I16::extract(table_data, OFFSET_Y_MAX),
            lowest_rec_ppem: U16::extract(table_data, OFFSET_LOWEST_PPEM),
            mac_style: U16::extract(table_data, OFFSET_MAC_STYLE),
            font_dir_hint: Self::parse_font_dir_hint(table_data),
            index_to_loc_fmt: Self::parse_index_to_loc_format(table_data),
            glyph_data_fmt: Self::parse_glyph_data_format(table_data),
        }
    }

    fn parse_font_dir_hint(table_data: &[u8]) -> FontDirectionHint {
        let value = I16::extract(table_data, OFFSET_FONT_DIR_HINT);
        match value {
            -2 => FontDirectionHint::RightToLeftWithNeutrals,
            -1 => FontDirectionHint::RightToLeft,
            0 => FontDirectionHint::MixedDirection,
            1 => FontDirectionHint::LeftToRight,
            2 => FontDirectionHint::LeftToRightWithNeutrals,
            _ => FontDirectionHint::Unknown(value),
        }
    }

    fn parse_index_to_loc_format(table_data: &[u8]) -> IndexToLocFormat {
        let value = I16::extract(table_data, OFFSET_INDEX_TO_LOC_FMT);
        match value {
            0 => IndexToLocFormat::ShortOffset,
            1 => IndexToLocFormat::LongOffset,
            _ => IndexToLocFormat::Unknown(value),
        }
    }

    fn parse_glyph_data_format(table_data: &[u8]) -> GlyphDataFormat {
        let value = I16::extract(table_data, OFFSET_GLYPH_DATA_FMT);
        match value {
            0 => GlyphDataFormat::CurrentFormat,
            _ => GlyphDataFormat::Unknown(value),
        }
    }
}

#[derive(Debug)]
pub enum FontDirectionHint {
    MixedDirection,
    LeftToRight,
    LeftToRightWithNeutrals,
    RightToLeft,
    RightToLeftWithNeutrals,
    Unknown(i16),
}

#[derive(Debug, Copy, Clone)]
pub enum IndexToLocFormat {
    ShortOffset,
    LongOffset,
    Unknown(i16),
}

#[derive(Debug)]
pub enum GlyphDataFormat {
    CurrentFormat,
    Unknown(i16),
}
