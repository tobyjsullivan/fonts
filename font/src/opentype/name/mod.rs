mod encoding;
mod name;
mod platform;
mod record;
mod table;

pub use self::table::NameTable;

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
