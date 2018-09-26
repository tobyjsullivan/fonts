mod iso;
mod macintosh;
mod unicode;
mod windows;

pub use self::iso::ISOEncoding;
pub use self::macintosh::MacintoshEncoding;
pub use self::unicode::UnicodeEncoding;
pub use self::windows::WindowsEncoding;
pub use super::platform::Platform;

#[derive(Debug, PartialEq)]
pub enum Encoding {
    Unicode { encoding: UnicodeEncoding },
    Macintosh { encoding: MacintoshEncoding },
    ISO { encoding: ISOEncoding },
    Windows { encoding: WindowsEncoding },
    Custom { encoding: u16 },
}

impl Encoding {
    pub fn lookup(platform: Platform, encoding_id: u16) -> Option<Encoding> {
        match platform {
            Platform::Unicode => Some(Encoding::Unicode {
                encoding: UnicodeEncoding::lookup(encoding_id)?,
            }),
            Platform::Macintosh => Some(Encoding::Macintosh {
                encoding: MacintoshEncoding::lookup(encoding_id)?,
            }),
            Platform::ISO => Some(Encoding::ISO {
                encoding: ISOEncoding::lookup(encoding_id)?,
            }),
            Platform::Windows => Some(Encoding::Windows {
                encoding: WindowsEncoding::lookup(encoding_id)?,
            }),
            Platform::Custom => Some(Encoding::Custom {
                encoding: encoding_id,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_platform_macintosh() {
        let platform = Platform::lookup(1);

        assert_eq!(platform, Some(Platform::Macintosh));
    }

    #[test]
    fn lookup_encoding_mac_roman() {
        let encoding = Encoding::lookup(Platform::Macintosh, 0);

        assert_eq!(
            encoding,
            Some(Encoding::Macintosh {
                encoding: MacintoshEncoding::Roman
            })
        );
    }
}
