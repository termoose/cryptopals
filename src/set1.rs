extern crate base64;
use hex::{FromHex, ToHex, FromHexError};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn hex_to_base64(s: &str) -> Result<String, FromHexError> {
    let hex_bytes = Vec::from_hex(s)?;
    Ok(base64::encode(&hex_bytes))
}

pub fn fixed_xor(a: &str, b: &str) -> Result<String, FromHexError> {
    let (a_bytes, b_bytes) = (Vec::from_hex(a)?, Vec::from_hex(b)?);

    let xor: Vec<u8> = a_bytes.iter()
        .zip(b_bytes.iter())
        .map(|(a, b)| a ^ b)
        .collect();
    
    Ok(xor.encode_hex::<String>())
}

pub fn crack(s: &str) -> (i32, String) {
    let test: HashMap<i32, String> = (0..128).map(|c| {
        let cs_hex = vec![c as u8; s.len()].encode_hex::<String>();
        let result = fixed_xor(s, &cs_hex).unwrap();
        let message = String::from_utf8(Vec::from_hex(result).unwrap());

        match message {
            Ok(msg) => (language_score(&msg), msg),
            Err(_error) => (-1, "".to_string())
        }
    }).collect();

    // Clear text is the one with the highest language score
    let score = test.keys().max().unwrap();
    let clear_text = &test[score];
    return (*score, clear_text.to_string())
}

pub fn crack_file(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let cracked_lines: HashMap<i32, String> = reader
        .lines()
        .map(|l| crack(&l.unwrap()))
        .collect();

    let clear = &cracked_lines[cracked_lines.keys().max().unwrap()];
    return Ok(clear.to_string())
}

fn language_score(s: &str) -> i32 {
    let caps_range = 'A'..'Z';
    let small_range = 'a'..'z';

    let score = s.chars().fold(0, |mut score, c| -> i32 {
        match caps_range.contains(&c) || small_range.contains(&c) || c == 32 as char {
            true => score += 1,
            false => score -= 1,
        }

        score
    });

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let answer = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(input).unwrap(), answer);
    }

    #[test]
    fn challenge2() {
        let (a, b) = ("1c0111001f010100061a024b53535009181c",
                      "686974207468652062756c6c277320657965");
        assert_eq!(fixed_xor(a, b).unwrap(), "746865206b696420646f6e277420706c6179")
    }

    #[test]
    fn challenge3() {
        let (_, solution) = crack("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        assert_eq!(solution, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn challenge4() {
        let solution = crack_file("4.txt").unwrap();
        assert_eq!(solution, "Now that the party is jumping\n");
    }

    #[test]
    fn language() {
        assert_eq!(language_score("abc___"), 0);
        assert_eq!(language_score("abcdef"), 6);
    }
}
