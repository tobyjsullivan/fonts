/// Pre-defined metadata fields which apply to all fonts regardless of platform.
/// Not all valid name IDs necessarily correspond to a defined field.
/// Find details for all of these in the MS docs: https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Name {
    CopyrightNotice,
    FontFamilyName,
    FontSubfamilyName,
    UniqueFontID,
    FullFontName,
    VersionString,
    PostScriptName,
    Trademark,
    Manufacturer,
    Designer,
    Description,
    VendorUrl,
    DesignerUrl,
    License,
    LicenseInfoUrl,
    TypographicFamilyName,
    TypographicSubfamilyName,
    CompatibleFullName, // Macintosh only
    SampleText,
    PostScriptCIDFindFontName,
    WWSFamilyName,
    WWSSubfamilyName,
    LightBackgroundPalette,
    DarkBackgroundPalette,
    VariationsPostScriptNamePrefix,
}

impl Name {
    pub fn lookup(name_id: u16) -> Option<Name> {
        match name_id {
            0 => Some(Name::CopyrightNotice),
            1 => Some(Name::FontFamilyName),
            2 => Some(Name::FontSubfamilyName),
            3 => Some(Name::UniqueFontID),
            4 => Some(Name::FullFontName),
            5 => Some(Name::VersionString),
            6 => Some(Name::PostScriptName),
            7 => Some(Name::Trademark),
            8 => Some(Name::Manufacturer),
            9 => Some(Name::Designer),
            10 => Some(Name::Description),
            11 => Some(Name::VendorUrl),
            12 => Some(Name::DesignerUrl),
            13 => Some(Name::License),
            14 => Some(Name::LicenseInfoUrl),
            16 => Some(Name::TypographicFamilyName),
            17 => Some(Name::TypographicSubfamilyName),
            18 => Some(Name::CompatibleFullName),
            19 => Some(Name::SampleText),
            20 => Some(Name::PostScriptCIDFindFontName),
            21 => Some(Name::WWSFamilyName),
            22 => Some(Name::WWSSubfamilyName),
            23 => Some(Name::LightBackgroundPalette),
            24 => Some(Name::DarkBackgroundPalette),
            25 => Some(Name::VariationsPostScriptNamePrefix),
            _ => None,
        }
    }
}
