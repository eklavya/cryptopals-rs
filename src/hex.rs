use std::ops::Shr;

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

pub fn hex_to_bytes(inp: &str) -> Vec<u8> {
    // println!("inp length is {}", inp.len());
    inp.as_bytes()
        .chunks_exact(2)
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
