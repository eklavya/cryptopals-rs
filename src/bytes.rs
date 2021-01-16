use rayon::iter::IntoParallelIterator;

use crate::base64::{encode_base64, B64};
use crate::eng::{char_freq_score, eng_socre};
use crate::hex::{Hex, HEX};
use rayon::prelude::*;
use openssl::symm::{Crypter, Mode, Cipher};

pub const ALL_CHARS: [char; 95] = [
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F',
    'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
    'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', ' ',
];

pub fn edit_distance_normalized(s1: &[u8], s2: &[u8]) -> u32 {
    s1.iter()
        .zip(s2.iter())
        .fold(0, |diff, (b1, b2)| diff + (b1 ^ b2).count_ones())
        / s1.len() as u32
}

#[derive(Debug)]
pub struct Bytes(pub Vec<u8>);

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
        B64(encode_base64(&self.0))
    }

    pub fn xor(&self, other: &Bytes) -> Bytes {
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
        ALL_CHARS
            .par_iter()
            .map(|c| {
                let un_xored = self.xor_with_char(*c);
                let res_str = un_xored.into_string();
                let score = char_freq_score(res_str.as_str()) + eng_socre(res_str.as_str());
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
        let key_sizes: Vec<(usize, u32)> = (1..41)
            .map(|s: usize| {
                if self.0.len() < s * 2 {
                    (s, u32::max_value())
                } else {
                    let chunks = self.0.chunks_exact(s * 2);
                    let (total, score) = chunks.fold((0, 0), |(total, score), b| {
                        let s1 = &b[..(s)];
                        let s2 = &b[s..(s * 2)];
                        (total + 1, score + edit_distance_normalized(s1, s2))
                    });
                    (s, score / total)
                }
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

    pub fn pad_pkcs(mut self, block_size: u8) -> Self {
        let rem = (self.0.len() % block_size as usize) as u8;
        let pad = if rem == 0 { 0 } else { block_size - rem };
        (0..pad).for_each(|_| {
            self.0.push(b'\x04');
        });
        self
    }

    pub fn encrypt_CBC(self, key: &[u8], iv: &[u8]) -> Bytes {
        let cipher = Cipher::aes_128_ecb();
        let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv)).unwrap();
        crypter.pad(false);
        let padded = self.pad_pkcs(key.len() as u8);
        let mut res = vec![0; padded.0.len() + key.len()];
        let (_, count) = padded.0.chunks_exact(key.len()).fold((Bytes(iv.into()), 0), |(xor_with, count), b| {
            let added = crypter.update(Bytes(b.into()).xor(&xor_with).0.as_slice(), &mut res[count..]).unwrap();
            (Bytes(res[count..{count + added}].to_vec()), count + added)
        });
        crypter.finalize(&mut res[count..]);
        res.truncate(count);
        Bytes(res)
    }

    pub fn decrypt_CBC(self, key: &[u8], iv: &[u8]) -> Bytes {
        let cipher = Cipher::aes_128_ecb();
        let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv)).unwrap();
        crypter.pad(false);
        let padded = self.pad_pkcs(key.len() as u8);
        let mut res = vec![0; padded.0.len() + key.len()];
        let (_, count) = padded.0.chunks_exact(key.len()).fold((iv, 0), |(xor_with, count), b| {
           let added = crypter.update(b, &mut res[count..]).unwrap();
            for i in 0..added {
                res[count + i] = res[count + i] ^ xor_with[i];
            }
            (b, count + added)
        });
        crypter.finalize(&mut res[count..]);
        res.truncate(count);
        Bytes(res)
    }
}

#[cfg(test)]
mod tests {
    use openssl::symm::{decrypt, encrypt, Cipher};

    use crate::bytes::Bytes;

    #[test]
    fn test_pad_pkcs() {
        let bytes = Bytes::from_string("YELLOW SUBMARINE".into());
        let new = bytes.pad_pkcs(20);
        assert_eq!(new.into_string(), "YELLOW SUBMARINE\x04\x04\x04\x04");
        let bytes = Bytes::from_string("YELLOW SUBMARINE".into());
        let new = bytes.pad_pkcs(9);
        assert_eq!(new.into_string(), "YELLOW SUBMARINE\x04\x04");
        let bytes = Bytes::from_string("YELLOW SUBMARINE".into());
        let new = bytes.pad_pkcs(16);
        assert_eq!(new.into_string(), "YELLOW SUBMARINE");
    }

    #[test]
    fn test_decrypt() {
        let key: &[u8; 16] = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
        let iv: &[u8; 16] = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
        assert_eq!(Bytes(b"Some Crypto TextSome Crypto Text".to_vec()).encrypt_CBC(key, iv)
                       .decrypt_CBC(key, iv).into_string(), "Some Crypto TextSome Crypto Text");
    }
}
