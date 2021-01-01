mod base64;
mod bytes;
mod eng;
mod hex;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::base64::B64;
    use crate::bytes::Bytes;
    use crate::eng::{char_freq_score, log_weight_score};
    use crate::hex::Hex;

    #[test]
    fn test_encode() {
        assert_eq!(Bytes::from_string("".into()).to_b64().0, "");
        assert_eq!(Bytes::from_string("f".into()).to_b64().0, "Zg==");
        assert_eq!(Bytes::from_string("fo".into()).to_b64().0, "Zm8=");
        assert_eq!(Bytes::from_string("foo".into()).to_b64().0, "Zm9v");
        assert_eq!(Bytes::from_string("foob".into()).to_b64().0, "Zm9vYg==");
        assert_eq!(Bytes::from_string("fooba".into()).to_b64().0, "Zm9vYmE=");
        assert_eq!(Bytes::from_string("foobar".into()).to_b64().0, "Zm9vYmFy");
        assert_eq!(B64("".into()).to_bytes().into_string(), "");
        assert_eq!(B64("Zg==".into()).to_bytes().into_string(), "f");
        assert_eq!(B64("Zm8=".into()).to_bytes().into_string(), "fo");
        assert_eq!(B64("Zm9v".into()).to_bytes().into_string(), "foo");
        assert_eq!(B64("Zm9vYg==".into()).to_bytes().into_string(), "foob");
        assert_eq!(B64("Zm9vYmE=".into()).to_bytes().into_string(), "fooba");
        assert_eq!(B64("Zm9vYmFy".into()).to_bytes().into_string(), "foobar");
        assert_eq!(
            Hex(
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".into()).to_b64().0,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
        assert_eq!(
            "I'm killing your brain like a poisonous mushroom",
            Bytes::from_string("I'm killing your brain like a poisonous mushroom".into())
                .to_b64()
                .to_bytes()
                .into_string()
        );
        assert_eq!(
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f69693a2a3c632d2c223769302a242631212a2b2e632025302069222b2d630c692b202831653a2c282c372d202d226926293a266b690a36692a316921202a22303a26653d2b20693337262437282e65203065272c3169202a3b31202a37652631652030652037652b26262836362c633121266539312a2e312424632c3a63262631372c20316921303d630c692228692d2a3d63222c3731202d22692a3169312c2e2b3176022b2d633c2636653e2c302527653d2b2c2728653d2b243d6331212a3669342a3c2f2169212069262b2636222163273c3765272c69692a316e30652d2623202d2c3d262930632b2637652c2d2a3c242d69372a6929303a37652122332c6331212a3667",
        Hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f69693a2a3c632d2c223769302a242631212a2b2e632025302069222b2d630c692b202831653a2c282c372d202d226926293a266b690a36692a316921202a22303a26653d2b20693337262437282e65203065272c3169202a3b31202a37652631652030652037652b26262836362c633121266539312a2e312424632c3a63262631372c20316921303d630c692228692d2a3d63222c3731202d22692a3169312c2e2b3176022b2d633c2636653e2c302527653d2b2c2728653d2b243d6331212a3669342a3c2f2169212069262b2636222163273c3765272c69692a316e30652d2623202d2c3d262930632b2637652c2d2a3c242d69372a6929303a37652122332c6331212a3667".into()).to_b64().to_hex().0
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
        assert_eq!("Now that the party is jumping\n", res.1);
    }

    #[test]
    fn challenge5() {
        assert_eq!(
            Bytes::from_string("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal.".into()).xor_with_key("ICE").to_hex().0,
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f6b"
        );
        assert_eq!(
            Hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f6b".into()).to_bytes().xor_with_key(
                    "ICE").into_string(),
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal."
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
        let lines = Hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f6b".into());
        let content = lines.to_bytes();
        assert_eq!(
            vec![
                2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 22, 23, 24, 25, 26,
                27, 28, 29, 30, 32, 33, 34, 35, 36, 37
            ],
            content.guess_keysize()
        );
    }

    #[test]
    fn challenge6() {
        let content = B64(BufReader::new(File::open("resources/chal6").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .collect::<String>());
        let inp = content.to_bytes();
        let key_sizes = inp.guess_keysize();
        let res = key_sizes
            .iter()
            .map(|key_size| {
                let key = inp.guess_key(*key_size);
                let out = inp.xor_with_key(key.as_str()).into_string();
                let score = log_weight_score(out.to_ascii_lowercase().as_str()).round() as u64;
                (key, out, score)
            })
            .max_by_key(|r| r.2)
            .unwrap();
        assert_eq!("I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that\'s my DJ Deshay cuttin\' all them Z\'s \nHittin\' hard and the girlies goin\' crazy \nVanilla\'s on the mike, man I\'m not lazy. \n\nI\'m lettin\' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse\'s to the side yellin\', Go Vanilla Go! \n\nSmooth \'cause that\'s the way I will be \nAnd if you don\'t give a damn, then \nWhy you starin\' at me \nSo get off \'cause I control the stage \nThere\'s no dissin\' allowed \nI\'m in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n\' play \n\nStage 2 -- Yea the one ya\' wanna listen to \nIt\'s off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI\'m an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI\'m like Samson -- Samson to Delilah \nThere\'s no denyin\', You can try to hang \nBut you\'ll keep tryin\' to get my style \nOver and over, practice makes perfect \nBut not if you\'re a loafer. \n\nYou\'ll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I\'m comin\' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin\' \nVanilla Ice is sellin\' and you people are buyin\' \n\'Cause why the freaks are jockin\' like Crazy Glue \nMovin\' and groovin\' trying to sing along \nAll through the ghetto groovin\' this here song \nNow you\'re amazed by the VIP posse. \n\nSteppin\' so hard like a German Nazi \nStartled by the bases hittin\' ground \nThere\'s no trippin\' on mine, I\'m just gettin\' down \nSparkamatic, I\'m hangin\' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\'89 in my time! You, \'90 is my year. \n\nYou\'re weakenin\' fast, YO! and I can tell it \nYour body\'s gettin\' hot, so, so I can smell it \nSo don\'t be mad and don\'t be sad \n\'Cause the lyrics belong to ICE, You can call me Dad \nYou\'re pitchin\' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don\'t be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you\'re dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n", res.1);
    }

    // #[test]
    // fn dummy() {
    //     let content = B64(BufReader::new(File::open("resources/chal6").unwrap())
    //         .lines()
    //         .map(|l| l.unwrap())
    //         .collect::<String>());
    //     let inp = content.to_bytes();
    //     println!(
    //         "{}",
    //         inp.xor_with_key("Terminator X: Bring the noise")
    //             .into_string()
    //     );
    // }
}
