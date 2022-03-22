extern crate rustc_serialize as serialize;
use serialize::hex::{FromHex, ToHex};

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
mod tests {
    use super::*;
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
