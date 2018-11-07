use opentype;

pub enum Name {
    CopyrightNotice,
    FontFamilyName,
    VersionString,
    FontSubfamilyName,
}

impl Name {
    pub fn into_opentype(&self) -> Option<opentype::tables::name::Name> {
        match self {
            Name::CopyrightNotice => Some(opentype::tables::name::Name::CopyrightNotice),
            Name::FontFamilyName => Some(opentype::tables::name::Name::FontFamilyName),
            Name::FontSubfamilyName => Some(opentype::tables::name::Name::FontSubfamilyName),
            Name::VersionString => Some(opentype::tables::name::Name::VersionString),
        }
    }
}
