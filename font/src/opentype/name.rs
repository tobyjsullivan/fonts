/// The name table stores strings which represent various metadata in the font (e.g., foundry name, font name, etc.).
use byteorder::{ByteOrder, BigEndian};

const FORMAT_OFFSET: usize = 0x00;
const U16_LENGTH: usize = 2;

#[derive(Debug)]
pub struct NameTable {
  pub format: Format,
}

impl NameTable {
  pub fn deserialize(data: &[u8]) -> Result<Self, ParseError> {
    Ok(NameTable{
      format: Self::parse_format(data)?,
    })
  }

  fn parse_format(data: &[u8]) -> Result<Format, ParseError> {
    match BigEndian::read_u16(&data[FORMAT_OFFSET..FORMAT_OFFSET+U16_LENGTH]) {
      0 => Ok(Format::Format0),
      1 => Ok(Format::Format1),
      _ => Err(ParseError::UnknownFormat),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Format {
  Format0,
  Format1,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
  UnknownFormat,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_format_0() {
    let mut data = vec![0x00u8; 32];
    data[0..2].clone_from_slice(&[0x00, 0x00]);

    assert_eq!(NameTable::parse_format(&data), Ok(Format::Format0));
  }
}
