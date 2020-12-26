mod eng;
mod hex;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::{BitAnd, BitOr, Shl, Shr};
use crate::eng::eng_socre;
use crate::hex::BASE64;

#[cfg(test)]
mod tests {
    use crate::eng::eng_socre;
    use crate::{single_byte_xor, xor_hex, get_max_score};
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use crate::hex::{hex_to_base64, hex_to_bytes};

    #[test]
    fn test_encode() {
        assert_eq!(
            hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn test_xor_hex() {
        assert_eq!(
            xor_hex(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965",
            ),
            "746865206b696420646f6e277420706c6179"
        );
    }

    #[test]
    fn decipher_xor() {
        if let Some((max, _)) = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
        ]
        .iter()
        .map(|c| {
            let un_xored = single_byte_xor(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
                *c,
            );
            let res_str = String::from_utf8(hex_to_bytes(un_xored.as_str())).unwrap();
            let score = eng_socre(res_str.as_str());
            (res_str, score.round() as u64)
        })
        .max_by_key(|x| x.1)
        {
            assert_eq!(max, "Cooking MC\'s like a pound of bacon");
        } else {
            assert_eq!(1, 2);
        };
    }

    #[test]
    fn challenge4() {
        let res = BufReader::new(File::open("resources/chal4").unwrap())
            .lines()
            .flat_map(|l| {
                get_max_score(l.as_ref().unwrap().as_str())
            })
            .max_by_key(|s| (*s).1).unwrap();
        assert_eq!("Now that the party is jumping\n", res.0);
    }
}

pub fn xor_hex(hex1: &str, hex2: &str) -> String {
    let h1 = hex::hex_to_bytes(hex1);
    let h2 = hex::hex_to_bytes(hex2);
    let v = h1.iter().zip(h2.iter()).map(|(a, b)| a ^ b).collect();
    hex::bytes_to_hex(v)
}

pub fn single_byte_xor(inp: &str, c: char) -> String {
    let cy = [c];
    let other = hex::bytes_to_hex(
        cy.iter()
            .cycle()
            .take(inp.len() / 2)
            .collect::<String>()
            .into_bytes(),
    );
    xor_hex(inp, other.as_str())
}

pub fn get_max_score(inp: &str) -> Option<(String, u64)> {
    BASE64
        .iter()
        .map(|c| {
            let un_xored = single_byte_xor(
                inp,
                *c,
            );
            let res_str = hex::hex_to_bytes(un_xored.as_str()).iter().map(|b| char::from(*b)).collect::<String>();
            let score = eng_socre(res_str.as_str());
            (res_str, score.round() as u64)
        })
        .max_by_key(|x| x.1)
}

// pub fn xor_with(it: &str, other: &str) -> String {
//     other.chars().cycle()
// }