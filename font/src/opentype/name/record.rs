use super::encoding::Encoding;
use super::name::Name;
use super::platform::Platform;
use byteorder::{BigEndian, ByteOrder};

const U16_LENGTH: usize = 2;

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

    pub fn deserialize(data: &[u8]) -> Result<Self, ParseError> {
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
        Encoding::lookup(platform, encoding_id).ok_or(ParseError::UnknownEncodingID)
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


#[cfg(test)]
mod tests {
    use super::*;
    use opentype::name::encoding::MacintoshEncoding;

    const SAMPLE_NAME_RECORD: [u8; 12] = [0u8, 1, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0];

    #[test]
    fn deserialize_name_record() {
        let result = NameRecord::deserialize(&SAMPLE_NAME_RECORD);
        let record = result.unwrap();

        assert_eq!(record.platform, Platform::Macintosh);
        assert_eq!(
            record.encoding,
            Encoding::Macintosh {
                encoding: MacintoshEncoding::Roman
            }
        );
        assert_eq!(record.language_id, 0u16);
        assert_eq!(record.name_id, 0);
        assert_eq!(record.name, Some(Name::CopyrightNotice));
        assert_eq!(record.string_length, 47);
        assert_eq!(record.string_offset, 0);
    }
}
