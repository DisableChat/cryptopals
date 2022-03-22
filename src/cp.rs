extern crate rustc_serialize as serialize;
use core::str::*;
use serialize::base64::{self, ToBase64};
use serialize::hex::{FromHex, ToHex};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::*;
use std::str;

pub static S1C4_FILE: &'static str = "./data/set1_challenge4.txt";
pub static S1C6_FILE: &'static str = "./data/set1_challenge6.txt";

pub fn hamming_distance(x: &[u8], y: &[u8]) -> Result<u32, String> {
    if x.len() != y.len() {
        return Err("Inputs do not have same length".into());
    }
    let tmp = x.xor(y);
    let mut rip: u32 = 0;
    for i in tmp {
        rip += u32::from(nonzero_bits_count(i));
    }
    Ok(rip)
}

pub fn nonzero_bits_count(mut u_8: u8) -> u8 {
    let mut result = 0u8;
    for _ in 0..8 {
        result += u_8 % 2;
        u_8 >>= 1;
    }
    result
}

#[allow(dead_code)]
pub fn hex_to_base64_as_string(hex_input: &str) -> String {
    match hex_input.from_hex() {
        Ok(result) => return result.to_base64(base64::STANDARD),
        Err(e) => return String::from("Error converting hex to base64 ") + &e.to_string(),
    }
}

#[allow(dead_code)]
pub fn fixed_xor(hex_input_one: &str, hex_input_two: &str) -> String {
    let mut result = Vec::new();
    let hex_input_one = hex_input_one.from_hex().expect("fixed xor | arg 1 invalid");
    let hex_input_two = hex_input_two.from_hex().expect("fixed xor | arg 2 invalid");

    for (pos, _e) in hex_input_one.iter().enumerate() {
        let i = hex_input_one[pos] ^ hex_input_two[pos];
        result.push(i);
    }
    return result.to_hex();
}

// Probably need to have different Type for the String in Result<(), String>
// Since its not really usefull
#[allow(dead_code)]
pub fn single_byte_xor_cipher(input: &str) -> Result<(char, u8), String> {
    // Originally I just picked letters that were the most common in the English language
    // (anything above ~6.5%) Which gave decent results but then I saw the ETAOI joke in the
    // Challenge Achievement unlock and used that
    let char_freq: Vec<u8> = "etaoinshrdlu ETAOINSHRDLU".as_bytes().to_vec();
    let input = input.from_hex().expect("from hex error | arg 1 invalid");
    let mut high_score = 0;
    let mut key: char = ' ';

    for x in 0..=u8::MAX {
        let mut current_score = 0;
        let mut u8_msg_vec = Vec::new();

        // Scoring in this case is pretty simple, where if the popular char is in the decoded message then +1 score
        // ----
        // However another idea is possibly just giving a weighted score of
        // each possible char value. Where the higher the occurence in the English language the higher the value
        // of the char itself, and vise versa. This would allow all characters to have the possibility to add
        // to the decoded messages score. Where the highest decoded message score sets the value the key.
        for i in &input {
            let val = i ^ x;
            if char_freq.contains(&val) {
                current_score += 1;
            }
            u8_msg_vec.push(val);
        }

        if current_score > high_score {
            high_score = current_score;
            key = x as char;
        }
    }
    Ok((key, high_score))
}

#[allow(dead_code)]
pub fn orignal_message_as_string(key: &char, message: &str) -> Result<String, Utf8Error> {
    let u8_msg_vec = message.from_hex().expect("from hex error | arg 1 invalid");
    let mut u8_decoded_msg_vec = Vec::new();
    let decoded_message: &str;

    for letter in &u8_msg_vec {
        u8_decoded_msg_vec.push(letter ^ (*key as u8));
    }

    match str::from_utf8(&u8_decoded_msg_vec) {
        Ok(res) => {
            decoded_message = res;
            Ok(decoded_message.to_string())
        }
        Err(e) => Err(e),
    }
}

// TODO: Make a struct to hold the from hex string, key and secret message
// This could allow possibly using rust combinators?
pub fn single_character_xor_detect(filepath: &str) -> Result<String, std::io::Error> {
    println!("this is the file {}", filepath);
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    // Using a tupple to hold a decoded message's key, key_weight, and decoded_message
    let mut decoded_messages: Vec<(char, u8, String)> = Vec::new();

    // *Probably* a better way to do this rather than have so many loops/indents
    // Possibly look into combinators or something with mapping?
    for line in reader.lines() {
        match line {
            Ok(contents) => {
                if let Ok((key, key_weight)) = single_byte_xor_cipher(&contents) {
                    if let Ok(msg) = orignal_message_as_string(&key, &contents) {
                        //println!("Key {}, Key Weight {}\nDecoded Message: {}", key, key_weight, msg);
                        decoded_messages.push((key, key_weight, msg));
                    };
                };
            }
            Err(e) => println!("Xor detect failed: {}", e),
        }
    }
    let mut best_key_weight = 0;
    let mut secret_message: String = "".to_string();

    // Similar to single_byte_xor_cipher(input: &str) we need a highscore Key
    // In this case however, since we have multiple decoded messages, we select
    // the key with the highest weight to select the correct 'secret message'
    for i in decoded_messages {
        if i.1 > best_key_weight {
            best_key_weight = i.1;
            secret_message = i.2;
        }
    }
    // Secret message still has newline, prob wana remove that at some point rip
    Ok(secret_message)
}

pub trait XOR {
    fn xor(&self, _: &Self) -> Vec<u8>;
}

impl XOR for [u8] {
    fn xor(&self, key: &[u8]) -> Vec<u8> {
        let mut result = self.to_vec();
        for chunk in result.chunks_mut(key.len()) {
            let len = chunk.len();
            for (i, &e) in chunk.iter_mut().zip(key[..len].iter()) {
                *i ^= e;
            }
        }
        result
    }
}

#[cfg(test)]
mod set_one {
    use super::*;
    #[test]
    fn test_hex_to_base64_as_string() {
        let input: &str     = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(hex_to_base64_as_string(input), expected_result);
    }

    #[test]
    fn test_fixed_xor() {
        let arg_one: &str = "1c0111001f010100061a024b53535009181c";
        let arg_two: &str = "686974207468652062756c6c277320657965";
        let expected_result = "746865206b696420646f6e277420706c6179";

        assert_eq!(fixed_xor(arg_one, arg_two), expected_result);
    }

    #[test]
    fn test_single_byte_xor_cipher() -> Result<(), String> {
        let expected_result = ('X', 23);
        let input: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        assert_eq!(single_byte_xor_cipher(input)?, expected_result);
        Ok(())
    }

    #[test]
    fn test_original_message_as_string() -> Result<(), Utf8Error> {
        let expected_result = "Cooking MC's like a pound of bacon";
        let key = 'X';
        let input: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        assert_eq!(orignal_message_as_string(&key, input)?, expected_result);
        Ok(())
    }

    #[test]
    fn test_single_character_xor_detect() -> Result<(), std::io::Error> {
        let expected_result = "Now that the party is jumping\n";
        assert_eq!(single_character_xor_detect(S1C4_FILE)?, expected_result);
        Ok(())
    }
    #[test]
    fn test_xor() {
        let key = b"ICE";
        let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        assert_eq!(
            "0b3637272a2b2e63622c2e69692a23693\
             a2a3c6324202d623d63343c2a26226324\
             272765272a282b2f20430a652e2c652a3\
             124333a653e2b2027630c692b20283165\
             286326302e27282f",
            &input.xor(key).to_hex()
        );
    }
}
