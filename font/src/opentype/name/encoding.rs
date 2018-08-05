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

#[derive(Debug, PartialEq)]
pub enum Encoding {
  Unicode { encoding: UnicodeEncoding },
  Macintosh { encoding: MacintoshEncoding },
  ISO { encoding: ISOEncoding },
  Windows { encoding: WindowsEncoding },
  Custom { encoding: u16 },
}

impl Encoding {
  pub fn lookup(platform: Platform, encoding_id: u16) -> Option<Encoding> {
    match platform {
      Platform::Unicode => Some(Encoding::Unicode {
        encoding: UnicodeEncoding::lookup(encoding_id)?,
      }),
      Platform::Macintosh => Some(Encoding::Macintosh {
        encoding: MacintoshEncoding::lookup(encoding_id)?,
      }),
      Platform::ISO => Some(Encoding::ISO {
        encoding: ISOEncoding::lookup(encoding_id)?,
      }),
      Platform::Windows => Some(Encoding::Windows {
        encoding: WindowsEncoding::lookup(encoding_id)?,
      }),
      Platform::Custom => Some(Encoding::Custom{ encoding: encoding_id }),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum UnicodeEncoding {
  Unicode1, // Deprecated, Unicode 1.0
  Unicode11, // Deprecated, Unicode 1.1
  ISO, // Deprecated, ISO/IEC 10646
  Unicode2BMP, // Unicode 2.0, BMP only (cmap subtable formats 0, 4, 6)
  Unicode2Full, // Unicode 2.0, full repertoire (cmap subtable formats 0, 4, 6, 10, 12)
  UnicodeVariation, // Unicode Variation Sequences (cmap subtable format 14)
  UnicodeFull, // Unicode full repertoire (cmap subtable formats 0, 4, 6, 10, 12, 13)
}

impl UnicodeEncoding {
  fn lookup(encoding_id: u16) -> Option<UnicodeEncoding> {
    match encoding_id {
      0 => Some(UnicodeEncoding::Unicode1),
      1 => Some(UnicodeEncoding::Unicode11),
      2 => Some(UnicodeEncoding::ISO),
      3 => Some(UnicodeEncoding::Unicode2BMP),
      4 => Some(UnicodeEncoding::Unicode2Full),
      5 => Some(UnicodeEncoding::UnicodeVariation),
      6 => Some(UnicodeEncoding::UnicodeFull),
      _ => None,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum MacintoshEncoding {
  Roman,
  Japanese,
  ChineseTraditional,
  Korean,
  Arabic,
  Hebrew,
  Greek,
  Russian,
  RSymbol,
  Devanagari,
  Gurmukhi,
  Gujarati,
  Oriya,
  Bengali,
  Tamil,
  Telugu,
  Kannada,
  Malayalam,
  Sinhalese,
  Burmese,
  Khmer,
  Thai,
  Laotian,
  Georgian,
  Armenian,
  ChineseSimplified,
  Tibetan,
  Mongolian,
  Geez,
  Slavic,
  Vietnamese,
  Sindhi,
  Uninterpreted,
}

impl MacintoshEncoding {
  fn lookup(encoding_id: u16) -> Option<MacintoshEncoding> {
    match encoding_id {
      0 => Some(MacintoshEncoding::Roman),
      1 => Some(MacintoshEncoding::Japanese),
      2 => Some(MacintoshEncoding::ChineseTraditional),
      3 => Some(MacintoshEncoding::Korean),
      4 => Some(MacintoshEncoding::Arabic),
      5 => Some(MacintoshEncoding::Hebrew),
      6 => Some(MacintoshEncoding::Greek),
      7 => Some(MacintoshEncoding::Russian),
      8 => Some(MacintoshEncoding::RSymbol),
      9 => Some(MacintoshEncoding::Devanagari),
      10 => Some(MacintoshEncoding::Gurmukhi),
      11 => Some(MacintoshEncoding::Gujarati),
      12 => Some(MacintoshEncoding::Oriya),
      13 => Some(MacintoshEncoding::Bengali),
      14 => Some(MacintoshEncoding::Tamil),
      15 => Some(MacintoshEncoding::Telugu),
      16 => Some(MacintoshEncoding::Kannada),
      17 => Some(MacintoshEncoding::Malayalam),
      18 => Some(MacintoshEncoding::Sinhalese),
      19 => Some(MacintoshEncoding::Burmese),
      20 => Some(MacintoshEncoding::Khmer),
      21 => Some(MacintoshEncoding::Thai),
      22 => Some(MacintoshEncoding::Laotian),
      23 => Some(MacintoshEncoding::Georgian),
      24 => Some(MacintoshEncoding::Armenian),
      25 => Some(MacintoshEncoding::ChineseSimplified),
      26 => Some(MacintoshEncoding::Tibetan),
      27 => Some(MacintoshEncoding::Mongolian),
      28 => Some(MacintoshEncoding::Geez),
      29 => Some(MacintoshEncoding::Slavic),
      30 => Some(MacintoshEncoding::Vietnamese),
      31 => Some(MacintoshEncoding::Sindhi),
      32 => Some(MacintoshEncoding::Uninterpreted),
      _ => None,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum ISOEncoding {
  Ascii, // 7-bit ASCII
  Iso10646, // ISO 10646
  Iso8859_1, // ISO 8859-1
}

impl ISOEncoding {
  fn lookup(encoding_id: u16) -> Option<ISOEncoding> {
    match encoding_id {
      0 => Some(ISOEncoding::Ascii),
      1 => Some(ISOEncoding::Iso10646),
      2 => Some(ISOEncoding::Iso8859_1),
      _ => None,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum WindowsEncoding {
  Symbol,
  UnicodeBMP, // UCS-2
  ShiftJIS,
  PRC,
  Big5,
  Wansung,
  Johab,
  UnicodeUCS4,
}

impl WindowsEncoding {
  fn lookup(encoding_id: u16) -> Option<WindowsEncoding> {
    match encoding_id {
      0 => Some(WindowsEncoding::Symbol),
      1 => Some(WindowsEncoding::UnicodeBMP),
      2 => Some(WindowsEncoding::ShiftJIS),
      3 => Some(WindowsEncoding::PRC),
      4 => Some(WindowsEncoding::Big5),
      5 => Some(WindowsEncoding::Wansung),
      6 => Some(WindowsEncoding::Johab),
      10 => Some(WindowsEncoding::UnicodeUCS4),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lookup_platform_macintosh() {
    let platform = Platform::lookup(1);

    assert_eq!(platform, Some(Platform::Macintosh));
  }

  #[test]
  fn lookup_encoding_mac_roman() {
    let encoding = Encoding::lookup(Platform::Macintosh, 0);

    assert_eq!(encoding, Some(Encoding::Macintosh{encoding: MacintoshEncoding::Roman}));
  }
}
