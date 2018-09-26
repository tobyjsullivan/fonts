/// See: https://docs.microsoft.com/en-us/typography/opentype/spec/name#platform-ids

#[derive(Debug, PartialEq)]
pub enum Platform {
  Unicode,
  Macintosh,
  ISO, // Deprecated as of OpenType spec v1.3.
  Windows,
  Custom,
}

impl Platform {
  pub fn lookup(platform_id: u16) -> Option<Platform> {
    match platform_id {
      0 => Some(Platform::Unicode),
      1 => Some(Platform::Macintosh),
      2 => Some(Platform::ISO),
      3 => Some(Platform::Windows),
      4 => Some(Platform::Custom),
      _ => None,
    }
  }
}
