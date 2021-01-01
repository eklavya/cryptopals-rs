use crate::bytes::Bytes;
use crate::hex::Hex;

pub fn encode_base64(inp: &Vec<u8>) -> String {
    let mut res = String::with_capacity((inp.len() / 3) * 4);
    for b in inp.chunks(3) {
        if b.len() == 3 {
            let d1 = b[0] >> 2;
            let d2 = ((b[0] << 6) | (b[1] >> 2)) >> 2;
            let d3 = ((b[1] << 4) | (b[2] >> 4)) >> 2;
            let d4 = (b[2] << 2) >> 2;
            res.push(BASE64[if (d1 as usize) < 64 { d1 as usize } else { 64 }]);
            res.push(BASE64[if (d2 as usize) < 64 { d2 as usize } else { 64 }]);
            res.push(BASE64[if (d3 as usize) < 64 { d3 as usize } else { 64 }]);
            res.push(BASE64[if (d4 as usize) < 64 { d4 as usize } else { 64 }]);
        } else {
            match b.len() {
                1 => {
                    let d1 = b[0] >> 2;
                    let d2 = (b[0] << 6) >> 2;
                    res.push(BASE64[if (d1 as usize) < 64 { d1 as usize } else { 64 }]);
                    res.push(BASE64[if (d2 as usize) < 64 { d2 as usize } else { 64 }]);
                    res.push('=');
                    res.push('=');
                }
                2 => {
                    let d1 = b[0] >> 2;
                    let d2 = ((b[0] << 6) | (b[1] >> 2)) >> 2;
                    let d3 = (b[1] << 4) >> 2;
                    res.push(BASE64[if (d1 as usize) < 64 { d1 as usize } else { 64 }]);
                    res.push(BASE64[if (d2 as usize) < 64 { d2 as usize } else { 64 }]);
                    res.push(BASE64[if (d3 as usize) < 64 { d3 as usize } else { 64 }]);
                    res.push('=');
                }
                _ => unreachable!(),
            }
        }
    }
    res
}

pub fn decode_base64(inp: &str) -> Vec<u8> {
    let mut res = Vec::with_capacity((inp.len() / 4) * 3);
    let bytes = inp.chars().map(|c| base64_to_byte(c)).collect::<Vec<u8>>();
    let mut chunks = bytes.chunks_exact(4).peekable();

    while let Some(b) = chunks.next() {
        if chunks.peek().is_some() {
            let b1 = (b[0] << 2) | (b[1] >> 4);
            let b2 = (b[1] << 4) | (b[2] >> 2);
            let b3 = (b[2] << 6) | b[3];
            res.push(b1);
            res.push(b2);
            res.push(b3);
        } else {
            if b[3] == 64 {
                if b[2] == 64 {
                    let b1 = (b[0] << 2) | (b[1] >> 4);
                    res.push(b1);
                } else {
                    let b1 = (b[0] << 2) | (b[1] >> 4);
                    let b2 = (b[1] << 4) | (b[2] >> 2);
                    res.push(b1);
                    res.push(b2);
                }
            } else {
                let b1 = (b[0] << 2) | (b[1] >> 4);
                let b2 = (b[1] << 4) | (b[2] >> 2);
                let b3 = (b[2] << 6) | b[3];
                res.push(b1);
                res.push(b2);
                res.push(b3);
            }
        }
    }
    res
}

const BASE64: [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/', '=',
];

const fn base64_to_byte(c: char) -> u8 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        'a' => 26,
        'b' => 27,
        'c' => 28,
        'd' => 29,
        'e' => 30,
        'f' => 31,
        'g' => 32,
        'h' => 33,
        'i' => 34,
        'j' => 35,
        'k' => 36,
        'l' => 37,
        'm' => 38,
        'n' => 39,
        'o' => 40,
        'p' => 41,
        'q' => 42,
        'r' => 43,
        's' => 44,
        't' => 45,
        'u' => 46,
        'v' => 47,
        'w' => 48,
        'x' => 49,
        'y' => 50,
        'z' => 51,
        '0' => 52,
        '1' => 53,
        '2' => 54,
        '3' => 55,
        '4' => 56,
        '5' => 57,
        '6' => 58,
        '7' => 59,
        '8' => 60,
        '9' => 61,
        '+' => 62,
        '/' => 63,
        _ => 64,
    }
}

pub struct B64(pub String);

impl B64 {
    pub fn to_bytes(&self) -> Bytes {
        Bytes(decode_base64(self.0.as_str()))
    }

    pub fn to_hex(&self) -> Hex {
        self.to_bytes().to_hex()
    }
}
