extern crate rustc_serialize as serialize;
use serialize::base64::{self, ToBase64};
use serialize::hex::FromHex;

pub fn hex_to_base64(hex_input: &str) -> String
{
    let result = hex_input.from_hex().unwrap().as_slice().to_base64(base64::STANDARD);
    return result;
}

fn main() {
    println!("Cryptopals!");
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_hex_to_base64() {
        let input: &str     = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(expected_result, hex_to_base64(input));
    }
}
