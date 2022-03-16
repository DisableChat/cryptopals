mod cp;

use cp::*;
use std::fs;

fn main() {
    println!("Cryptopals!");
    // Set 1  Challange 3
    let input: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    /*match cp::single_byte_xor_cipher(input) {
        Ok(key) => match cp::orignal_message_as_string(&key, input) {
            Ok(msg) => println!("Decoded Message: {}", msg),
            Err(e) => println!("Failed to decode original mssg: {}", e),
        },
        Err(e) => println!("Failed to find key for msg: {}", e),
    };
    */

    single_character_xor_detect(cp::S1C4_FILE);
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    single_character_xor_detect(cp::S1C4_FILE);
}
