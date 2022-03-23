use xor::XOR;

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
