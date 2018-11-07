use opentype;

pub enum Name {
    CopyrightNotice,
    FontFamilyName,
}

impl Name {
    pub fn into_opentype(&self) -> Option<opentype::tables::name::Name> {
        match self {
            Name::CopyrightNotice => Some(opentype::tables::name::Name::CopyrightNotice),
            Name::FontFamilyName => Some(opentype::tables::name::Name::FontFamilyName),
            _ => None,
        }
    }
}
