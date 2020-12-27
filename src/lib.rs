mod eng;
mod hex;
mod base64;

use crate::eng::eng_socre;
use crate::hex::{hex_to_bytes};
use itertools::Itertools;
use crate::base64::BASE64;

#[cfg(test)]
mod tests {
    use crate::eng::eng_socre;
    use crate::{
        edit_distance, get_max_score, guess_keysize, single_byte_xor, xor_hex, xor_with_key,
    };
    use itertools::Itertools;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};
    use crate::base64::{hex_to_base64, decode_base64, encode_base64};
    use crate::hex::hex_to_bytes;

    #[test]
    fn test_encode() {
        assert_eq!(
            hex_to_base64(
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
        assert_eq!(
            "I'm killing your brain like a poisonous mushroom",
            decode_base64(encode_base64("I'm killing your brain like a poisonous mushroom").as_str())
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
            .flat_map(|l| get_max_score(l.as_ref().unwrap().as_str()))
            .max_by_key(|s| (*s).1)
            .unwrap();
        assert_eq!("Now that the party is jumping\n", res.0);
    }

    #[test]
    fn challenge5() {
        assert_eq!(
            xor_with_key("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE"),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }

    #[test]
    fn edit_test() {
        assert_eq!(
            edit_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()),
            37
        );
    }

    #[test]
    fn guess_test() {
        let lines = BufReader::new(File::open("resources/chal6").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .take(10)
            .collect::<String>();
        // println!("{:?}", lines.chars().dropping(3).next());
        let content = lines.as_bytes();
        let mut inp = [0; 320];
        inp.copy_from_slice(&content[..320]);
        // println!("{}", String::from_utf8_lossy(&inp));
        // for i in &inp {
        //     println!("{}", (*i));
        // }
        // println!("{:?}", inp);
        println!("{}", guess_keysize(inp));
    }
}

pub fn xor_hex(hex1: &str, hex2: &str) -> String {
    let h1 = hex::hex_to_bytes(hex1);
    let h2 = hex::hex_to_bytes(hex2);
    let v = h1.iter().zip(h2.iter()).map(|(a, b)| a ^ b).collect();
    hex::bytes_to_hex(v)
}

pub fn xor_with_key(b1: &str, key: &str) -> String {
    hex::bytes_to_hex(
        b1.as_bytes()
            .iter()
            .zip(key.as_bytes().iter().cycle())
            .map(|(a, b)| a ^ b)
            .collect(),
    )
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
            let un_xored = single_byte_xor(inp, *c);
            let res_str = hex::hex_to_bytes(un_xored.as_str())
                .iter()
                .map(|b| char::from(*b))
                .collect::<String>();
            let score = eng_socre(res_str.as_str());
            (res_str, score.round() as u64)
        })
        .max_by_key(|x| x.1)
}

pub fn edit_distance(s1: &[u8], s2: &[u8]) -> u32 {
    s1.iter()
        .zip(s2.iter())
        .fold(0, |diff, (b1, b2)| diff + (b1 ^ b2).count_ones())
}

pub fn guess_keysize(inp: [u8; 320]) -> u32 {
    (2u32..40)
        .min_by_key(|size| {
            let score = inp
                .chunks((*size) as usize)
                .take(4)
                .collect::<Vec<&[u8]>>()
                .windows(2)
                .fold(0, |acc, s| {
                    // let s1 = slice_to_bytes(s[0]);
                    // let s2 = slice_to_bytes(s[1]);
                    // println!("{} {}", String::from_utf8(s1.clone()).unwrap(), String::from_utf8(s2.clone()).unwrap());
                    acc + (edit_distance(s[0], s[1]) / *size)
                });
            println!("{} : {}", size, score);
            score
        })
        .unwrap()
}
