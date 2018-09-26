pub mod encoding;
pub mod platform;

use byteorder::{BigEndian, ByteOrder};

use self::encoding::Encoding;
use self::platform::Platform;

const U16_LENGTH: usize = 2;

/// The name table stores strings which represent various metadata in
/// the font (e.g., foundry name, font name, etc.).
#[derive(Debug)]
pub struct NameTable {
    /// NameTable comes in two formats, 0 and 1.
    /// The latter supports language-tag records.
    pub format: Format,
    /// The number of name records in the table.
    pub count: u16,
    /// The offset of the string storage region (from the start of the table).
    pub string_offset: u16,
    /// The parsed name records.
    pub name_records: Vec<NameRecord>,
}

impl NameTable {
    const FORMAT_OFFSET: usize = 0;
    const COUNT_OFFSET: usize = 2;
    const STRING_OFFSET_OFFSET: usize = 4;
    const NAME_RECORDS_OFFSET: usize = 6;
    const NAME_RECORD_LENGTH: usize = 12;

    /// Deserialize the name table from font file data.
    pub fn deserialize(data: &[u8]) -> Result<Self, ParseError> {
        Ok(NameTable {
            format: Self::parse_format(data)?,
            count: Self::parse_record_count(data),
            string_offset: Self::parse_string_offset(data),
            name_records: Self::parse_name_records(data)?,
        })
    }

    fn parse_format(data: &[u8]) -> Result<Format, ParseError> {
        match BigEndian::read_u16(&data[Self::FORMAT_OFFSET..Self::FORMAT_OFFSET + U16_LENGTH]) {
            0 => Ok(Format::Format0),
            1 => Ok(Format::Format1),
            _ => Err(ParseError::UnknownFormat),
        }
    }

    fn parse_record_count(data: &[u8]) -> u16 {
        BigEndian::read_u16(&data[Self::COUNT_OFFSET..Self::COUNT_OFFSET + U16_LENGTH])
    }

    fn parse_string_offset(data: &[u8]) -> u16 {
        BigEndian::read_u16(
            &data[Self::STRING_OFFSET_OFFSET..Self::STRING_OFFSET_OFFSET + U16_LENGTH],
        )
    }

    fn parse_name_records(data: &[u8]) -> Result<Vec<NameRecord>, ParseError> {
        let mut records = vec![];
        let num_records = Self::parse_record_count(data);
        for n in 0..num_records {
            let offset = Self::NAME_RECORDS_OFFSET + n as usize * Self::NAME_RECORD_LENGTH;
            let record_data = &data[offset..offset + Self::NAME_RECORD_LENGTH];

            records.push(NameRecord::deserialize(record_data)?);
        }

        Ok(records)
    }
}

/// Errors which result from unexpected data.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Only formats 0 and 1 are recognized.
    /// Anything else will result in an UnknownFormat error.
    UnknownFormat,
    /// The Platform ID hasn't been implemented yet.
    UnknownPlatformID,
    /// The Encoding ID hasn't been implemented yet.
    UnknownEncodingID,
}

/// Supported formats for Name Tables.
#[derive(Debug, PartialEq)]
pub enum Format {
    Format0,
    Format1,
}

#[derive(Debug, PartialEq)]
pub struct NameRecord {
    platform: Platform,
    encoding: Encoding,
    language_id: u16,
    name_id: u16,
    name: Option<Name>,
    string_length: u16,
    string_offset: usize,
}

impl NameRecord {
    const PLATFORM_ID_OFFSET: usize = 0;
    const ENCODING_ID_OFFSET: usize = 2;
    const LANGUAGE_ID_OFFSET: usize = 4;
    const NAME_ID_OFFSET: usize = 6;
    const STRING_LENGTH_OFFSET: usize = 8;
    const STRING_OFFSET_OFFSET: usize = 10;

    fn deserialize(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            platform: Self::parse_platform(data)?,
            encoding: Self::parse_encoding(data)?,
            language_id: Self::parse_language_id(data),
            name_id: Self::parse_name_id(data),
            name: Name::lookup(Self::parse_name_id(data)),
            string_length: Self::parse_string_length(data),
            string_offset: Self::parse_string_offset(data),
        })
    }

    fn parse_platform(data: &[u8]) -> Result<Platform, ParseError> {
        let platform_id = BigEndian::read_u16(
            &data[Self::PLATFORM_ID_OFFSET..Self::PLATFORM_ID_OFFSET + U16_LENGTH],
        );
        Platform::lookup(platform_id).ok_or(ParseError::UnknownPlatformID)
    }

    fn parse_encoding(data: &[u8]) -> Result<Encoding, ParseError> {
        let platform = Self::parse_platform(data)?;
        let encoding_id = BigEndian::read_u16(
            &data[Self::ENCODING_ID_OFFSET..Self::ENCODING_ID_OFFSET + U16_LENGTH],
        );
        encoding::Encoding::lookup(platform, encoding_id).ok_or(ParseError::UnknownEncodingID)
    }

    fn parse_language_id(data: &[u8]) -> u16 {
        BigEndian::read_u16(&data[Self::LANGUAGE_ID_OFFSET..Self::LANGUAGE_ID_OFFSET + U16_LENGTH])
    }

    fn parse_name_id(data: &[u8]) -> u16 {
        BigEndian::read_u16(&data[Self::NAME_ID_OFFSET..Self::NAME_ID_OFFSET + U16_LENGTH])
    }

    fn parse_string_length(data: &[u8]) -> u16 {
        BigEndian::read_u16(
            &data[Self::STRING_LENGTH_OFFSET..Self::STRING_LENGTH_OFFSET + U16_LENGTH],
        )
    }

    fn parse_string_offset(data: &[u8]) -> usize {
        BigEndian::read_u16(
            &data[Self::STRING_OFFSET_OFFSET..Self::STRING_OFFSET_OFFSET + U16_LENGTH],
        ) as usize
    }
}

/// Pre-defined metadata fields which apply to all fonts regardless of platform.
/// Not all valid name IDs necessarily correspond to a defined field.
/// Find details for all of these in the MS docs: https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids
#[derive(Debug, PartialEq)]
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
    fn lookup(name_id: u16) -> Option<Name> {
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

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_HEADER: [u8; 6] = [0u8, 0, 0, 26, 1, 62];
    const SAMPLE_NAME_RECORD: [u8; 12] = [0u8, 1, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0];

    #[test]
    fn parse_format_0() {
        let mut data = vec![0u8; 1062];
        data[..6].clone_from_slice(&SAMPLE_HEADER);

        assert_eq!(NameTable::parse_format(&data), Ok(Format::Format0));
    }

    #[test]
    fn parse_record_count() {
        let mut data = vec![0u8; 1062];
        data[..6].clone_from_slice(&SAMPLE_HEADER);

        assert_eq!(NameTable::parse_record_count(&data), 26);
    }

    #[test]
    fn parse_string_offset() {
        let mut data = vec![0u8; 1062];
        data[..6].clone_from_slice(&SAMPLE_HEADER);

        assert_eq!(NameTable::parse_string_offset(&data), 318);
    }

    #[test]
    fn deserialize_name_record() {
        let result = NameRecord::deserialize(&SAMPLE_NAME_RECORD);
        let record = result.unwrap();

        assert_eq!(record.platform, Platform::Macintosh);
        assert_eq!(
            record.encoding,
            Encoding::Macintosh {
                encoding: encoding::MacintoshEncoding::Roman
            }
        );
        assert_eq!(record.language_id, 0u16);
        assert_eq!(record.name_id, 0);
        assert_eq!(record.name, Some(Name::CopyrightNotice));
        assert_eq!(record.string_length, 47);
        assert_eq!(record.string_offset, 0);
    }
}
