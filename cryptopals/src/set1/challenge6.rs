use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use xor::XOR;
extern crate base64;
extern crate rustc_serialize as serialize;
use base64::decode_config_buf;

#[allow(dead_code)]
pub static S1C6_FILE: &'static str = "cryptopals/data/set1_challenge6.txt";

const MIN_KEYSIZE: usize = 2;
const MAX_KEYSIZE: usize = 40;
const KEYSIZE_BLOCKS: usize = 4;

pub fn hamming_distance(x: &[u8], y: &[u8]) -> Result<u32, String> {
    if x.len() != y.len() {
        return Err("Inputs do not have same length".into());
    }
    let tmp = x.xor(y);
    let mut rip: u32 = 0;
    for i in tmp {
        rip += u32::from(differing_bit_count(i));
    }
    Ok(rip)
}

pub fn differing_bit_count(mut u_8: u8) -> u8 {
    let mut result = 0u8;
    for _ in 0..8 {
        result += u_8 % 2;
        u_8 >>= 1;
    }
    result
}

pub fn from_base64_file(path: &Path) -> Result<Vec<u8>, Error> {
    let mut result = Vec::<u8>::new();
    let file = File::open(&path)?;
    let buffer = BufReader::new(file);

    for line in buffer.lines() {
        decode_config_buf(line.unwrap().trim(), base64::STANDARD, &mut result).unwrap();
    }
    // base64::read::DecoderReader - map a read of b64 bytes to decoded bytes, wrap a reader(fil, network sockets, etc)
    Ok(result)
}

pub fn find_key_size(input: &[u8]) -> usize {
    // Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.
    let mut keysize_contenders: Vec<(usize, u32)> = Vec::new();

    for keysize in MIN_KEYSIZE..=MAX_KEYSIZE {
        keysize_contenders.push((keysize, normalized_key_size(input, keysize) as u32));
    }

    // Er we want to sort by the min hamming_distance in this instance
    //
    // TODO: use a fn that is more clear in sorting. Sorting by key and then using the min hamming_distance
    // seems a bit misleading because the key should be the key size?
    keysize_contenders.sort_by_key(|k| k.1);
    println!("Keysize contenders{:?}", keysize_contenders);

    // Get the first index value's keysize
    keysize_contenders[0].0
}

pub fn normalized_key_size(input: &[u8], key_size: usize) -> f32 {
    let chunks: Vec<&[u8]> = input.chunks(key_size).collect();
    let mut distance_sum: f32 = 0.0;

    for i in 0..KEYSIZE_BLOCKS {
        for j in i + 1..KEYSIZE_BLOCKS {
            distance_sum += hamming_distance(chunks[i], chunks[j]).expect("Rip the the ham") as f32;
        }
    }

    distance_sum as f32 / key_size as f32 * 100.0
}
