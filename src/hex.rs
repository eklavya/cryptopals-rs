use std::ops::Shr;

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

