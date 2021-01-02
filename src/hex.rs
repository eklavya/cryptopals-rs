use crate::base64::B64;
use crate::bytes::Bytes;

pub const fn hex_to_byte(h: u8) -> u8 {
    if h <= 57 {
        h - 48
    } else {
        h - 87
    }
}

pub const HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

#[derive(Debug)]
pub struct Hex(pub String);

impl Hex {
    pub fn to_bytes(&self) -> Bytes {
        Bytes(
            self.0
                .as_bytes()
                .chunks_exact(2)
                .map(|sl| hex_to_byte(sl[0]) * 16 + hex_to_byte(sl[1]))
                .collect::<Vec<u8>>(),
        )
    }

    pub fn to_b64(&self) -> B64 {
        self.to_bytes().to_b64()
    }
}
