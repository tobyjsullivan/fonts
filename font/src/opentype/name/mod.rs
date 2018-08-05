/// The name table stores strings which represent various metadata in the font (e.g., foundry name, font name, etc.).
pub mod encoding;

use byteorder::{ByteOrder, BigEndian};

use self::encoding::{Platform, Encoding};

const U16_LENGTH: usize = 2;

// Name Table header
const FORMAT_OFFSET: usize = 0;
const COUNT_OFFSET: usize = 2;
const STRING_OFFSET_OFFSET: usize = 4;
const NAME_RECORDS_OFFSET: usize = 6;

// Name Record
const NAME_RECORD_LENGTH: usize = 12;
const PLATFORM_ID_OFFSET: usize = 0;
const ENCODING_ID_OFFSET: usize = 2;

#[derive(Debug)]
pub struct NameTable {
  pub format: Format,
  pub count: u16,
  pub string_offset: u16,
  pub name_records: Vec<NameRecord>,
}

impl NameTable {
  pub fn deserialize(data: &[u8]) -> Result<Self, ParseError> {
    Ok(NameTable{
      format: Self::parse_format(data)?,
      count: Self::parse_record_count(data),
      string_offset: Self::parse_string_offset(data),
      name_records: Self::parse_name_records(data)?,
    })
  }

  fn parse_format(data: &[u8]) -> Result<Format, ParseError> {
    match BigEndian::read_u16(&data[FORMAT_OFFSET..FORMAT_OFFSET+U16_LENGTH]) {
      0 => Ok(Format::Format0),
      1 => Ok(Format::Format1),
      _ => Err(ParseError::UnknownFormat),
    }
  }

  fn parse_record_count(data: &[u8]) -> u16 {
    BigEndian::read_u16(&data[COUNT_OFFSET..COUNT_OFFSET+U16_LENGTH])
  }

  fn parse_string_offset(data: &[u8]) -> u16 {
    BigEndian::read_u16(&data[STRING_OFFSET_OFFSET..STRING_OFFSET_OFFSET+U16_LENGTH])
  }

  fn parse_name_records(data: &[u8]) -> Result<Vec<NameRecord>, ParseError> {
    let mut records = vec![];
    let num_records = Self::parse_record_count(data);
    for n in 0..num_records {
      let offset = NAME_RECORDS_OFFSET + n as usize * NAME_RECORD_LENGTH;
      let record_data = &data[offset..offset+NAME_RECORD_LENGTH];

      records.push(NameRecord::deserialize(record_data)?);
    }

    Ok(records)
  }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
  UnknownFormat,
  UnknownPlatformID,
  UnknownEncodingID,
}

#[derive(Debug, PartialEq)]
pub enum Format {
  Format0,
  Format1,
}

#[derive(Debug, PartialEq)]
pub struct NameRecord {
  platform: Platform,
  encoding: Encoding,
}

impl NameRecord {
  fn deserialize(data: &[u8]) -> Result<Self, ParseError> {
    Ok(Self {
      platform: Self::parse_platform(data)?,
      encoding: Self::parse_encoding(data)?,
    })
  }

  fn parse_platform(data: &[u8]) -> Result<Platform, ParseError> {
    let platform_id = BigEndian::read_u16(&data[PLATFORM_ID_OFFSET..PLATFORM_ID_OFFSET+U16_LENGTH]);
    encoding::Platform::lookup(platform_id).ok_or(ParseError::UnknownPlatformID)
  }

  fn parse_encoding(data: &[u8]) -> Result<Encoding, ParseError> {
    let platform = Self::parse_platform(data)?;
    let encoding_id = BigEndian::read_u16(&data[ENCODING_ID_OFFSET..ENCODING_ID_OFFSET+U16_LENGTH]);
    encoding::Encoding::lookup(platform, encoding_id).ok_or(ParseError::UnknownEncodingID)
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
    assert_eq!(record.encoding, Encoding::Macintosh{encoding: encoding::MacintoshEncoding::Roman});
  }
}
