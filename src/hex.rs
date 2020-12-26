use std::ops::Shr;
use std::slice::Iter;
use itertools::Itertools;

pub const BASE64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

const fn hex_to_byte(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => 128,
    }
}

const HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn hex_to_bytes(inp: &str) -> Vec<u8> {
    inp.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|sl| hex_to_byte(sl[0]) * 16 + hex_to_byte(sl[1]))
        .collect::<Vec<u8>>()
}

pub fn bytes_to_hex(inp: Vec<u8>) -> String {
    inp.iter()
        .flat_map(|b| {
            let place2 = HEX[(b / 16) as usize];
            let place1 = HEX[(b % 16) as usize];
            let v = vec![place2, place1];
            v.into_iter()
        })
        .collect()
}

pub fn hex_to_base64(inp: &str) -> String {
    let mut res = String::with_capacity((inp.len() / 3) * 4);
    for b in hex_to_bytes(inp).chunks_exact(3) {
        let d1 = b[0].shr(2);
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
