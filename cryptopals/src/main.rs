mod set1;
use set1::cp::*;

use set1::challenge6::*;
use std::path::Path;
use std::str;
fn main() {
    println!("Cryptopals!");

    let x = "this is a test".as_bytes();
    let y = "wokka wokka!!!".as_bytes();
    match hamming_distance(x, y) {
        Ok(res) => print!("result: {:?}\n", res),
        Err(e) => print!("Error: {}\n", e),
    };

    let contents =
        from_base64_file(Path::new(set1::challenge6::S1C6_FILE)).expect("rip yo contents boi");

    let key_size = find_key_size(contents.as_slice());
    println!("key_size {}", key_size);

    let mut transposed_blocks: Vec<Vec<u8>> = transpose_blocks(contents.as_slice(), key_size);
    for i in transposed_blocks {
        let tmp = single_byte_xor_cipher(str::from_utf8(&i).unwrap()).expect("rip");
        println!("tmp {:?} {:?}", tmp.0, tmp.1);
    }
    //transposed_blocks;
}
