use opentype::tables::name::Encoding;

pub fn lookup(encoding_id: u16) -> Option<Encoding> {
    match encoding_id {
        0 => Some(Encoding::MacintoshRoman),
        1 => Some(Encoding::MacintoshJapanese),
        2 => Some(Encoding::MacintoshChineseTraditional),
        3 => Some(Encoding::MacintoshKorean),
        4 => Some(Encoding::MacintoshArabic),
        5 => Some(Encoding::MacintoshHebrew),
        6 => Some(Encoding::MacintoshGreek),
        7 => Some(Encoding::MacintoshRussian),
        8 => Some(Encoding::MacintoshRSymbol),
        9 => Some(Encoding::MacintoshDevanagari),
        10 => Some(Encoding::MacintoshGurmukhi),
        11 => Some(Encoding::MacintoshGujarati),
        12 => Some(Encoding::MacintoshOriya),
        13 => Some(Encoding::MacintoshBengali),
        14 => Some(Encoding::MacintoshTamil),
        15 => Some(Encoding::MacintoshTelugu),
        16 => Some(Encoding::MacintoshKannada),
        17 => Some(Encoding::MacintoshMalayalam),
        18 => Some(Encoding::MacintoshSinhalese),
        19 => Some(Encoding::MacintoshBurmese),
        20 => Some(Encoding::MacintoshKhmer),
        21 => Some(Encoding::MacintoshThai),
        22 => Some(Encoding::MacintoshLaotian),
        23 => Some(Encoding::MacintoshGeorgian),
        24 => Some(Encoding::MacintoshArmenian),
        25 => Some(Encoding::MacintoshChineseSimplified),
        26 => Some(Encoding::MacintoshTibetan),
        27 => Some(Encoding::MacintoshMongolian),
        28 => Some(Encoding::MacintoshGeez),
        29 => Some(Encoding::MacintoshSlavic),
        30 => Some(Encoding::MacintoshVietnamese),
        31 => Some(Encoding::MacintoshSindhi),
        32 => Some(Encoding::MacintoshUninterpreted),
        _ => None,
    }
}
