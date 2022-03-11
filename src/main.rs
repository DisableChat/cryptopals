extern crate rustc_serialize as serialize;
use serialize::base64::{self, ToBase64};
use serialize::hex::{FromHex, ToHex};
use core::str;

pub fn print_hex_to_base64(hex_input: &str) -> String {

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

fn main() {
    println!("Cryptopals!");
}

#[cfg(test)]
mod set_1 {
    use super::*;

#[test]
    fn test_print_hex_to_base64() {
        let input: &str     = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(expected_result, print_hex_to_base64(input));
    }

#[test]
    fn test_fixed_xor() {
        let arg_one: &str = "1c0111001f010100061a024b53535009181c";
        let arg_two: &str ="686974207468652062756c6c277320657965";
        let expected_result = "746865206b696420646f6e277420706c6179";

        assert_eq!(expected_result, fixed_xor(arg_one, arg_two));
    }
}
