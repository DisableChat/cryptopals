mod cp;

use cp::*;

fn main() {
    println!("Cryptopals!");

    // Set 1 Challange 4
    if let Ok(decoded_msg) = single_character_xor_detect(cp::S1C4_FILE) {
        println!("Decoded Message: {}", decoded_msg);
    };

    let x = "this is a test".as_bytes();
    let y = "wokka wokka!!!".as_bytes();
    match hamming_distance(x, y) {
        Ok(res) => print!("result: {:?}", res),
        Err(e) => print!("Error: {}", e),
    };
}
