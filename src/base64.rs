use crate::hex::hex_to_bytes;

pub fn hex_to_base64(inp: &str) -> String {
    let mut res = String::with_capacity((inp.len() / 3) * 4);
    for b in hex_to_bytes(inp).chunks_exact(3) {
        let d1 = b[0] >> 2;
        let d2 = ((b[0] << 6) | (b[1] >> 2)) >> 2;
        let d3 = ((b[1] << 4) | (b[2] >> 4)) >> 2;
        let d4 = (b[2] << 2) >> 2;
        res.push(BASE64[d1 as usize]);
        res.push(BASE64[d2 as usize]);
        res.push(BASE64[d3 as usize]);
        res.push(BASE64[d4 as usize]);
    }
    res
}

pub fn encode_base64(inp: &str) -> String {
    let mut res = String::with_capacity((inp.len() / 3) * 4);
    for b in inp.as_bytes().chunks_exact(3) {
        let d1 = b[0] >> 2;
        let d2 = ((b[0] << 6) | (b[1] >> 2)) >> 2;
        let d3 = ((b[1] << 4) | (b[2] >> 4)) >> 2;
        let d4 = (b[2] << 2) >> 2;
        res.push(BASE64[d1 as usize]);
        res.push(BASE64[d2 as usize]);
        res.push(BASE64[d3 as usize]);
        res.push(BASE64[d4 as usize]);
    }
    res
}

pub fn decode_base64(inp: &str) -> String {
    let mut res = String::with_capacity((inp.len() / 4) * 3);
    for b in inp.chars()
        .map(|c| base64_to_byte(c))
        .collect::<Vec<u8>>()
        .chunks_exact(4) {
        let b1 = (b[0] << 2) | (b[1] >> 4);
        let b2 = (b[1] << 4) | (b[2] >> 2);
        let b3 = (b[2] << 6) | b[3];
        res.push(char::from(b1));
        res.push(char::from(b2));
        res.push(char::from(b3));
    }
    res
}

pub const BASE64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
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
        _ => 64
    }
}