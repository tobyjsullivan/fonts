use apple::roman::{map_to_ucs2, AppleRoman};

pub struct Ucs2 {
    bytes: Vec<u8>,
}

impl Ucs2 {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
}

impl From<AppleRoman> for Ucs2 {
    fn from(roman: AppleRoman) -> Self {
        let mut output: Vec<u8> = vec![];

        for byte in roman.to_bytes() {
            let mapped = map_to_ucs2(*byte);
            output.push(mapped[0]);
            output.push(mapped[1]);
        }

        Self {
            bytes: output.to_vec(),
        }
    }
}
