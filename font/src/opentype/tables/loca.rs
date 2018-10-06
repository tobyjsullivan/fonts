use super::maxp::Version;

#[derive(Debug)]
pub struct LocaTable {

}

impl LocaTable {
  pub fn parse(table_data: &[u8], version: Version, num_glyphs: u16) -> Self {
    // TODO: Can't parse until I know if it's short or long format.
    Self {}
  }
}
