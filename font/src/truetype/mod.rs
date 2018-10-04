use super::sfnt::SfntFile;

#[derive(Debug)]
pub struct TrueTypeFile<'a> {
    sfnt: SfntFile<'a>,
}

impl<'a> TrueTypeFile<'a> {
    pub fn deserialize(content: &'a [u8]) -> Self {
        Self {
            sfnt: SfntFile::deserialize(content),
        }
    }
}
