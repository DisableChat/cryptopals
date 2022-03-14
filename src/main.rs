extern crate rustc_serialize as serialize;
use serialize::base64::{self, ToBase64};
use serialize::hex::{FromHex, ToHex};
use core::str;

pub fn hex_to_base64_as_string(hex_input: &str) -> String {

    match hex_input.from_hex(){
        Ok(result) => return result.to_base64(base64::STANDARD),
        Err(e) => return String::from("Error converting hex to base64 " ) + &e.to_string()
    }
}

// Prob delete this
pub fn hex_to_base64(hex_input: &str) -> Vec<u8> {

    // Prob need to handle error a better way
    match hex_input.from_hex(){
        Ok(result) => return result,
        Err(_) => return Vec::new()
    }
}

pub fn fixed_xor(hex_input_one: &str, hex_input_two: &str) -> String {
    let mut result = Vec::new();

    //let hex_input_one = hex_to_base64(hex_input_one);
    //let hex_input_two = hex_to_base64(hex_input_two);
    let hex_input_one = hex_input_one.from_hex().expect("fixed xor | arg 1 invalid");
    let hex_input_two = hex_input_two.from_hex().expect("fixed xor | arg 2 invalid");

    for (pos, _e) in hex_input_one.iter().enumerate() {
        let i = hex_input_one[pos] ^ hex_input_two[pos];
        result.push(i);
    }

    return result.to_hex();
}

// Probably should of just uppercased the string so that the ETAOIN SHRDLU wouldn't need lowercase
// Also doesn't handle if there is a tie situation
pub fn single_byte_xor_cipher(input: &str) -> Result<char, String> {

    let alphabet: Vec<u8>    = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes().to_vec();
    let char_freq: Vec<u8>   = "etaoinshrdlu ETAOINSHRDLU".as_bytes().to_vec();

    let input                = input.from_hex().expect("from hex error | arg 1 invalid");
    let mut high_score       = 0;
    let mut key : char       = ' ';

    for letter in alphabet {
        let mut current_score = 0;
        let mut u8_msg_vec = Vec::new();

        for i in &input {
            let val = i ^ letter;
            if char_freq.contains(&val) {
               current_score+=1;
            }
            u8_msg_vec.push(val);
        }

        if high_score < current_score {
            high_score = current_score;
            key = letter as char;
        }
    }

    Ok(key)
}

pub fn orignal_message_as_string(key: &char, message: &str) -> Result<String, String> {

    let u8_msg_vec = message.from_hex().expect("from hex error | arg 1 invalid");
    let mut u8_decoded_msg_vec = Vec::new();
    let decoded_message: &str;

    for letter in &u8_msg_vec {
        u8_decoded_msg_vec.push(letter ^ (*key as u8));
    }

    decoded_message = match str::from_utf8(&u8_decoded_msg_vec) {
        Ok(res) => res,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    Ok(decoded_message.to_string())
}

fn main() {
    println!("Cryptopals!");

    // Set 1  Challange 3
    let input: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    match single_byte_xor_cipher(input) {
        Ok(key) => {
            match orignal_message_as_string(&key, input) {
                Ok(msg) => println!("Decoded Message: {}", msg),
                Err(e) => println!("Failed to decode original mssg: {}", e)
            }
        },
        Err(e) => println!("Failed to find key for msg: {}", e)
    };
}

#[cfg(test)]
mod set_1 {
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
        let arg_two: &str ="686974207468652062756c6c277320657965";
        let expected_result = "746865206b696420646f6e277420706c6179";

        assert_eq!(fixed_xor(arg_one, arg_two), expected_result);
    }

#[test]
    fn test_single_byte_xor_cipher() -> Result<(), String> {
        let expected_result = 'X';
        let input: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        assert_eq!(single_byte_xor_cipher(input)?, expected_result);
        Ok(())
    }

#[test]
    fn test_original_message_as_string() -> Result<(), String> {
        let expected_result = "Cooking MC's like a pound of bacon";
        let key = 'X';
        let input: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        assert_eq!(orignal_message_as_string(&key, input)?, expected_result);
        Ok(())
    }
}
