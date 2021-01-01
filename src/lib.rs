mod base64;
mod eng;
mod hex;

use crate::base64::{base64_to_byte, hex_to_base64, BASE64};
use crate::eng::{char_freq_score, eng_socre, log_weight_score};
use crate::hex::{bytes_to_hex, hex_to_byte, hex_to_bytes, HEX};
use rayon::prelude::*;
use std::collections::HashSet;
use std::slice::Iter;

#[cfg(test)]
mod tests {
    use crate::base64::{decode_base64, encode_base64, hex_to_base64, BASE64};
    use crate::eng::{char_freq_score, eng_socre, log_weight_score};
    use crate::hex::{bytes_to_hex, hex_to_bytes};
    use crate::{Bytes, Hex, B64};
    use std::convert::TryFrom;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};

    #[test]
    fn test_encode() {
        assert_eq!(
            hex_to_base64(
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
        assert_eq!(
            "I'm killing your brain like a poisonous mushroom",
            decode_base64(
                encode_base64("I'm killing your brain like a poisonous mushroom").as_str()
            )
        );
        assert_eq!(
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f69693a2a3c632d2c223769302a242631212a2b2e632025302069222b2d630c692b202831653a2c282c372d202d226926293a266b690a36692a316921202a22303a26653d2b20693337262437282e65203065272c3169202a3b31202a37652631652030652037652b26262836362c633121266539312a2e312424632c3a63262631372c20316921303d630c692228692d2a3d63222c3731202d22692a3169312c2e2b3176022b2d633c2636653e2c302527653d2b2c2728653d2b243d6331212a3669342a3c2f2169212069262b2636222163273c3765272c69692a316e30652d2623202d2c3d262930632b2637652c2d2a3c242d69372a6929303a37652122332c6331212a3667",
        decode_base64(
            encode_base64("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f69693a2a3c632d2c223769302a242631212a2b2e632025302069222b2d630c692b202831653a2c282c372d202d226926293a266b690a36692a316921202a22303a26653d2b20693337262437282e65203065272c3169202a3b31202a37652631652030652037652b26262836362c633121266539312a2e312424632c3a63262631372c20316921303d630c692228692d2a3d63222c3731202d22692a3169312c2e2b3176022b2d633c2636653e2c302527653d2b2c2728653d2b243d6331212a3669342a3c2f2169212069262b2636222163273c3765272c69692a316e30652d2623202d2c3d262930632b2637652c2d2a3c242d69372a6929303a37652122332c6331212a3667").as_str()
        )
        );
    }

    #[test]
    fn test_xor_hex() {
        assert_eq!(
            Hex("1c0111001f010100061a024b53535009181c".into())
                .to_bytes()
                .xor(Hex("686974207468652062756c6c277320657965".into()).to_bytes())
                .to_hex()
                .0,
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
            let un_xored =
                Hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".into())
                    .to_bytes()
                    .xor_with_char(*c);
            let res_str = un_xored.into_string();
            let score = char_freq_score(res_str.as_str()); // + eng_socre(res_str.as_str());
            (res_str, score)
        })
        .max_by_key(|x| x.1)
        {
            // assert_eq!(max_score, 11);
            assert_eq!(max, "Cooking MC\'s like a pound of bacon");
        } else {
            assert_eq!(1, 2);
        };
    }

    #[test]
    fn challenge4() {
        let res = BufReader::new(File::open("resources/chal4").unwrap())
            .lines()
            .flat_map(|l| Hex(l.unwrap()).to_bytes().get_max_score())
            .max_by_key(|s| (*s).2)
            .unwrap();
        println!("{:?}", res);
        assert_eq!("Now that the party is jumping\n", res.1);
    }

    #[test]
    fn challenge5() {
        assert_eq!(
            Bytes::from_string("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal, you hear something else and I hear something else. Is it because the program is not correct or is it because the program is correct but I am not getting it right?\
            And you would think that this would be enough but no, it's definitely not enough to just have this.".into()).xor_with_key("ICE").to_hex().0,
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f69693a2a3c632d2c223769302a242631212a2b2e632025302069222b2d630c692b202831653a2c282c372d202d226926293a266b690a36692a316921202a22303a26653d2b20693337262437282e65203065272c3169202a3b31202a37652631652030652037652b26262836362c633121266539312a2e312424632c3a63262631372c20316921303d630c692228692d2a3d63222c3731202d22692a3169312c2e2b3176022b2d633c2636653e2c302527653d2b2c2728653d2b243d6331212a3669342a3c2f2169212069262b2636222163273c3765272c69692a316e30652d2623202d2c3d262930632b2637652c2d2a3c242d69372a6929303a37652122332c6331212a3667"
        );
        assert_eq!(
            Hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f69693a2a3c632d2c223769302a242631212a2b2e632025302069222b2d630c692b202831653a2c282c372d202d226926293a266b690a36692a316921202a22303a26653d2b20693337262437282e65203065272c3169202a3b31202a37652631652030652037652b26262836362c633121266539312a2e312424632c3a63262631372c20316921303d630c692228692d2a3d63222c3731202d22692a3169312c2e2b3176022b2d633c2636653e2c302527653d2b2c2728653d2b243d6331212a3669342a3c2f2169212069262b2636222163273c3765272c69692a316e30652d2623202d2c3d262930632b2637652c2d2a3c242d69372a6929303a37652122332c6331212a3667".into()).to_bytes().xor_with_key(
                    "ICE").into_string(),
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal, you hear something else and I hear something else. Is it because the program is not correct or is it because the program is correct but I am not getting it right?\
            And you would think that this would be enough but no, it's definitely not enough to just have this."
        );
    }

    #[test]
    fn edit_test() {
        assert_eq!(
            Bytes::from_string("this is a test".into())
                .edit_distance(Bytes::from_string("wokka wokka!!!".into())),
            37
        );
    }

    #[test]
    fn guess_test() {
        let lines = B64(BufReader::new(File::open("resources/chal6").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .collect::<String>());
        // let lines = Hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f69693a2a3c632d2c223769302a242631212a2b2e632025302069222b2d630c692b202831653a2c282c372d202d226926293a266b690a36692a316921202a22303a26653d2b20693337262437282e65203065272c3169202a3b31202a37652631652030652037652b26262836362c633121266539312a2e312424632c3a63262631372c20316921303d630c692228692d2a3d63222c3731202d22692a3169312c2e2b3176022b2d633c2636653e2c302527653d2b2c2728653d2b243d6331212a3669342a3c2f2169212069262b2636222163273c3765272c69692a316e30652d2623202d2c3d262930632b2637652c2d2a3c242d69372a6929303a37652122332c6331212a3667".into());
        let content = lines.to_bytes();
        // let mut inp = [0; 320];
        // inp.copy_from_slice(&content[..320]);
        println!("{:?}", content.guess_keysize());
    }

    #[test]
    fn challenge6() {
        let content = B64(BufReader::new(File::open("resources/chal6").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .collect::<String>());
        let inp = content.to_bytes();
        let key_sizes = inp.guess_keysize();
        // println!("debased is {} long", debased.len());
        // println!("hex is {} long", inp.len());
        // println!("hex is {}", inp);
        let res = key_sizes
            .iter()
            .map(|key_size| {
                let key = inp.guess_key(*key_size);
                let out = inp.xor_with_key(key.as_str()).into_string();
                let score = log_weight_score(out.to_ascii_lowercase().as_str()).round() as u64;
                (key, out, score)
            })
            .max_by_key(|r| r.2);
        // for i in 1.
        println!("{:?}", res);
        // println!("{}", String::from_utf8(hex_to_bytes(xor_with_key(inp.as_str(), "rwru").as_str())).unwrap());
    }

    #[test]
    fn dummy() {
        let content = B64(BufReader::new(File::open("resources/chal6").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .collect::<String>());
        let inp = content.to_bytes();
        println!("{}", inp.xor_with_key("Terminator X: Bring the noise").into_string());
    }
}

pub struct Hex(String);
pub struct B64(String);
pub struct Bytes(Vec<u8>);

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
        let mut res = String::with_capacity((self.0.len() / 3) * 4);
        for b in self.to_bytes().0.chunks_exact(3) {
            let d1 = b[0] >> 2;
            let d2 = ((b[0] << 6) | (b[1] >> 2)) >> 2;
            let d3 = ((b[1] << 4) | (b[2] >> 4)) >> 2;
            let d4 = (b[2] << 2) >> 2;
            res.push(BASE64[d1 as usize]);
            res.push(BASE64[d2 as usize]);
            res.push(BASE64[d3 as usize]);
            res.push(BASE64[d4 as usize]);
        }
        B64(res)
    }
}

impl B64 {
    pub fn to_bytes(&self) -> Bytes {
        let mut res: Vec<u8> = Vec::with_capacity((self.0.len() / 4) * 3);
        let mut bytes = self
            .0
            .chars()
            .map(|c| base64_to_byte(c))
            .collect::<Vec<u8>>();
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
        Bytes(res)
    }

    pub fn to_hex(&self) -> Hex {
        self.to_bytes().to_hex()
    }

    pub fn xor(&self, other: Hex) -> Hex {
        self.to_bytes().xor(other.to_bytes()).to_hex()
    }
}

pub const ALLOWED_CHARS: [char; 56] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' ', '.', '?', ':'
];

impl Bytes {
    pub fn to_hex(&self) -> Hex {
        Hex(self
            .0
            .iter()
            .flat_map(|b| {
                let place2 = HEX[(b / 16) as usize];
                let place1 = HEX[(b % 16) as usize];
                let v = vec![place2, place1];
                v.into_iter()
            })
            .collect())
    }

    pub fn to_b64(&self) -> B64 {
        let mut res = String::with_capacity((self.0.len() / 3) * 4);
        for b in self.0.chunks(3) {
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
        B64(res)
    }

    pub fn xor(&self, other: Bytes) -> Bytes {
        Bytes(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a ^ b)
                .collect::<Vec<u8>>(),
        )
    }

    pub fn xor_with_key(&self, key: &str) -> Bytes {
        Bytes(
            self.0
                .iter()
                .zip(key.as_bytes().iter().cycle())
                .map(|(a, b)| a ^ b)
                .collect(),
        )
    }

    pub fn xor_with_byte(&self, b: u8) -> Bytes {
        Bytes(
            self.0
                .iter()
                .zip([b].iter().cycle())
                .map(|(b1, b2)| b1 ^ b2)
                .collect::<Vec<u8>>(),
        )
    }

    pub fn xor_with_char(&self, c: char) -> Bytes {
        let mut bytes = [0; 4];
        c.encode_utf8(&mut bytes);
        self.xor_with_byte(bytes[0])
    }

    pub fn into_string(self) -> String {
        self.0.iter().map(|b| char::from(*b)).collect::<String>()
    }

    pub fn get_max_score(&self) -> Option<(char, String, u64)> {
        ALLOWED_CHARS
            .iter()
            .map(|c| {
                let un_xored = self.xor_with_char(*c);
                let res_str = un_xored.into_string();
                let score = char_freq_score(res_str.as_str()) + eng_socre(res_str.as_str());
                // if score > 8 {
                // println!("char {} with {} , score: {}", *c, res_str, score);
                // }
                (*c, res_str, score)
            })
            .max_by_key(|x| x.2)
    }

    pub fn edit_distance(&self, other: Bytes) -> u32 {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(0, |diff, (b1, b2)| diff + (b1 ^ b2).count_ones())
    }

    pub fn guess_key(&self, key_size: u8) -> String {
        (0..key_size)
            .into_par_iter()
            .flat_map(|size| {
                let mut it = self.0.iter();
                (0..size).for_each(|_| {
                    it.next();
                });
                let some = Bytes(
                    it.step_by(key_size as usize)
                        .map(|a| *a)
                        .collect::<Vec<u8>>(),
                );
                some.get_max_score()
            })
            .map(|t| t.0)
            .collect::<String>()
    }

    pub fn guess_keysize(&self) -> Vec<u8> {
        let key_sizes: Vec<(usize, u32)> = (1..51)
            .map(|s: usize| {
                let chunks = self.0.chunks_exact(s * 2);
                let (total, score) = chunks.fold((0, 0), |(total, score), b| {
                    let s1 = &b[..(s)];
                    let s2 = &b[s..(s * 2)];
                    // println!("{}", bytes_to_hex(s1.to_vec()));
                    // println!("{}", bytes_to_hex(s2.to_vec()));
                    (total + 1, score + (edit_distance(s1, s2) / (s as u32)))
                });
                println!("for key size {} score {}", s, score / total);
                (s, score / total)
            })
            .collect::<Vec<_>>();
        let min_score = key_sizes.iter().min_by_key(|e| e.1).unwrap().1;
        key_sizes
            .iter()
            .filter(|e| e.1 == min_score)
            .map(|e| e.0 as u8)
            .collect::<Vec<u8>>()
    }

    pub fn from_string(s: String) -> Self {
        Bytes(s.into_bytes())
    }
}

pub fn edit_distance(s1: &[u8], s2: &[u8]) -> u32 {
    s1.iter()
        .zip(s2.iter())
        .fold(0, |diff, (b1, b2)| diff + (b1 ^ b2).count_ones())
}
