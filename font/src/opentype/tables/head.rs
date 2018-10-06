use byteorder::{BigEndian, ByteOrder};

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

type Datetime = i64;

#[derive(Debug)]
pub struct HeadTable<'a> {
  major_version: u16,
  minor_version: u16,
  font_revision: u32, // TODO: This should be a 32-bit fixed point number
  checksum_adjustment: u32,
  magic_number: &'a [u8],
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
  index_to_loc_fmt: IndexToLocFormat,
  glyph_data_fmt: GlyphDataFormat,
}

impl<'a> HeadTable<'a> {
  pub fn parse(table_data: &'a [u8]) -> Self {
    Self {
      major_version: Self::parse_major_version(table_data),
      minor_version: Self::parse_minor_version(table_data),
      font_revision: Self::parse_font_revision(table_data),
      checksum_adjustment: Self::parse_checksum_adjustment(table_data),
      magic_number: Self::parse_magic_number(table_data),
      flags: Self::parse_flags(table_data),
      units_per_em: Self::parse_units_per_em(table_data),
      created: Self::parse_created(table_data),
      modified: Self::parse_modified(table_data),
      x_min: Self::parse_x_min(table_data),
      y_min: Self::parse_y_min(table_data),
      x_max: Self::parse_x_max(table_data),
      y_max: Self::parse_y_max(table_data),
      lowest_rec_ppem: Self::parse_lowest_rec_ppem(table_data),
      mac_style: Self::parse_mac_style(table_data),
      font_dir_hint: Self::parse_font_dir_hint(table_data),
      index_to_loc_fmt: Self::parse_index_to_loc_format(table_data),
      glyph_data_fmt: Self::parse_glyph_data_format(table_data),
    }
  }

  fn parse_major_version(table_data: &[u8]) -> u16 {
    BigEndian::read_u16(&table_data[OFFSET_MAJOR_VERSION..OFFSET_MAJOR_VERSION + 2])
  }

  fn parse_minor_version(table_data: &[u8]) -> u16 {
    BigEndian::read_u16(&table_data[OFFSET_MINOR_VERSION..OFFSET_MINOR_VERSION + 2])
  }

  fn parse_font_revision(table_data: &[u8]) -> u32 {
    BigEndian::read_u32(&table_data[OFFSET_FONT_REVISION..OFFSET_FONT_REVISION + 4])
  }

  fn parse_checksum_adjustment(table_data: &[u8]) -> u32 {
    BigEndian::read_u32(&table_data[OFFSET_CHECKSUM_ADJ..OFFSET_CHECKSUM_ADJ + 4])
  }

  fn parse_magic_number(table_data: &'a [u8]) -> &'a [u8] {
    &table_data[OFFSET_MAGIC_NUMBER..OFFSET_MAGIC_NUMBER + 4]
  }

  fn parse_flags(table_data: &[u8]) -> u16 {
    BigEndian::read_u16(&table_data[OFFSET_FLAGS..OFFSET_FLAGS+2])
  }

  fn parse_units_per_em(table_data: &[u8]) -> u16 {
    BigEndian::read_u16(&table_data[OFFSET_UNITS_PER_EM..OFFSET_UNITS_PER_EM+2])
  }

  fn parse_created(table_data: &[u8]) -> Datetime {
    BigEndian::read_i64(&table_data[OFFSET_CREATED..OFFSET_CREATED + 8])
  }

  fn parse_modified(table_data: &[u8]) -> Datetime {
    BigEndian::read_i64(&table_data[OFFSET_MODIFIED..OFFSET_MODIFIED + 8])
  }

  fn parse_x_min(table_data: &[u8]) -> i16 {
    BigEndian::read_i16(&table_data[OFFSET_X_MIN..OFFSET_X_MIN + 2])
  }

  fn parse_y_min(table_data: &[u8]) -> i16 {
    BigEndian::read_i16(&table_data[OFFSET_Y_MIN..OFFSET_Y_MIN + 2])
  }

  fn parse_x_max(table_data: &[u8]) -> i16 {
    BigEndian::read_i16(&table_data[OFFSET_X_MAX..OFFSET_X_MAX + 2])
  }

  fn parse_y_max(table_data: &[u8]) -> i16 {
    BigEndian::read_i16(&table_data[OFFSET_Y_MAX..OFFSET_Y_MAX + 2])
  }

  fn parse_lowest_rec_ppem(table_data: &[u8]) -> u16 {
    BigEndian::read_u16(&table_data[OFFSET_LOWEST_PPEM..OFFSET_LOWEST_PPEM + 2])
  }

  fn parse_mac_style(table_data: &[u8]) -> u16 {
    BigEndian::read_u16(&table_data[OFFSET_MAC_STYLE..OFFSET_MAC_STYLE + 2])
  }

  fn parse_font_dir_hint(table_data: &[u8]) -> FontDirectionHint {
    let value = BigEndian::read_i16(&table_data[OFFSET_FONT_DIR_HINT..OFFSET_FONT_DIR_HINT + 2]);
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
    let value = BigEndian::read_i16(&table_data[OFFSET_INDEX_TO_LOC_FMT..OFFSET_INDEX_TO_LOC_FMT + 2]);
    match value {
      0 => IndexToLocFormat::ShortOffset,
      1 => IndexToLocFormat::LongOffset,
      _ => IndexToLocFormat::Unknown(value),
    }
  }

  fn parse_glyph_data_format(table_data: &[u8]) -> GlyphDataFormat {
    let value = BigEndian::read_i16(&table_data[OFFSET_GLYPH_DATA_FMT..OFFSET_GLYPH_DATA_FMT + 2]);
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

#[derive(Debug)]
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
