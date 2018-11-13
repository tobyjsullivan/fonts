use apple::roman::AppleRoman;
use unicode::ucs2::Ucs2;

pub struct Utf8 {
    bytes: Vec<u8>,
}

impl Utf8 {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
}

impl From<AppleRoman> for Utf8 {
    fn from(roman: AppleRoman) -> Self {
        let ucs2: Ucs2 = roman.into();
        ucs2.into()
    }
}

impl From<Ucs2> for Utf8 {
    fn from(input: Ucs2) -> Self {
        let mut output: Vec<u8> = vec![];
        let bytes = input.to_bytes();

        for i in 0..bytes.len() / 2 {
            let b0 = bytes[i * 2];
            let b1 = bytes[i * 2 + 1];
            let c = ((b0 as u16) << 8) + (b1 as u16);

            if c < 0x80 {
                output.push(c as u8);
            } else if c < 0x0800 {
                output.push(0b11000000 | (0b00011111 & (c >> 6)) as u8);
                output.push(0b10000000 | (0b00111111 & c) as u8);
            } else {
                output.push(0b11100000 | (0b00001111 & (c >> 12)) as u8);
                output.push(0b10000000 | (0b00111111 & (c >> 6)) as u8);
                output.push(0b10000000 | (0b00111111 & c) as u8);
            }
        }

        Self {
            bytes: output.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_from_roman() {
        let roman = AppleRoman::from_bytes(&[66, 108, 97, 99, 107, 115, 119, 111, 114, 100]);
        let utf8 = Utf8::from(roman);

        let string = String::from_utf8(utf8.to_bytes().to_vec())
            .expect("Failed to get UTF8 string from bytes.");

        assert_eq!(string, "Blacksword");
    }
}
