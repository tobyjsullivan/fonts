mod iso;
mod macintosh;
mod unicode;
mod windows;

pub use super::platform::Platform;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Encoding {
    // ISO and Unicode Encodings
    Ascii,            // 7-bit ASCII
    ISO10646,         // ISO 10646
    ISO8859_1,        // ISO 8859-1
    Unicode1,         // Deprecated, Unicode 1.0
    Unicode11,        // Deprecated, Unicode 1.1
    Unicode2BMP,      // Unicode 2.0, BMP only (cmap subtable formats 0, 4, 6)
    Unicode2Full,     // Unicode 2.0, full repertoire (cmap subtable formats 0, 4, 6, 10, 12)
    UnicodeVariation, // Unicode Variation Sequences (cmap subtable format 14)
    UnicodeFull,      // Unicode full repertoire (cmap subtable formats 0, 4, 6, 10, 12, 13)

    // Apple Encodings
    MacintoshRoman,
    MacintoshJapanese,
    MacintoshChineseTraditional,
    MacintoshKorean,
    MacintoshArabic,
    MacintoshHebrew,
    MacintoshGreek,
    MacintoshRussian,
    MacintoshRSymbol,
    MacintoshDevanagari,
    MacintoshGurmukhi,
    MacintoshGujarati,
    MacintoshOriya,
    MacintoshBengali,
    MacintoshTamil,
    MacintoshTelugu,
    MacintoshKannada,
    MacintoshMalayalam,
    MacintoshSinhalese,
    MacintoshBurmese,
    MacintoshKhmer,
    MacintoshThai,
    MacintoshLaotian,
    MacintoshGeorgian,
    MacintoshArmenian,
    MacintoshChineseSimplified,
    MacintoshTibetan,
    MacintoshMongolian,
    MacintoshGeez,
    MacintoshSlavic,
    MacintoshVietnamese,
    MacintoshSindhi,
    MacintoshUninterpreted,

    // Microsoft Encodings
    WindowsSymbol,
    WindowsUnicodeBMP, // UCS-2
    WindowsShiftJIS,
    WindowsPRC,
    WindowsBig5,
    WindowsWansung,
    WindowsJohab,
    WindowsUnicodeUCS4,

    Custom { encoding: u16 },
}

impl Encoding {
    pub fn lookup(platform: Platform, encoding_id: u16) -> Option<Encoding> {
        match platform {
            Platform::Unicode => unicode::lookup(encoding_id),
            Platform::Macintosh => macintosh::lookup(encoding_id),
            Platform::ISO => iso::lookup(encoding_id),
            Platform::Windows => windows::lookup(encoding_id),
            Platform::Custom => Some(Encoding::Custom {
                encoding: encoding_id,
            }),
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

        assert_eq!(encoding, Some(Encoding::MacintoshRoman));
    }
}
