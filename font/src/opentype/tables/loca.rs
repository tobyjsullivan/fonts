use super::maxp::Version;
use opentype::types::{DataType, Offset};

#[derive(Debug)]
pub struct LocaTable {
  offsets: Vec<Offset>,
}

impl LocaTable {
  pub fn parse(table_data: &[u8], version: Version, num_glyphs: u16) -> Self {
    let mut offsets = Vec::new();
    // TODO: Can't parse until I know if it's short or long format.
    Self { offsets }
  }
}
