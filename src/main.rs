mod cp;

use cp::*;

fn main() {
    println!("Cryptopals!");

    // Set 1 Challange 4
    if let Ok(decoded_msg) = single_character_xor_detect(cp::S1C4_FILE) {
        println!("Decoded Message: {}", decoded_msg);
    };
}
