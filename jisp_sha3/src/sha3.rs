//! The functions in this module perform their respective variations of the SHA-3 algorithm and include message padding 
use crate::internals::sponge::keccak_c;

pub fn sha3_224(m:&Vec<u8>) -> Vec<u8> {
    let suffix = vec![false, true];
    keccak_c::<18>(m, &suffix, 224)
}
pub fn sha3_256(m:&Vec<u8>) -> Vec<u8> {
    let suffix = vec![false, true];
    keccak_c::<17>(m, &suffix, 256)
}
pub fn sha3_384(m:&Vec<u8>) -> Vec<u8> {
    let suffix = vec![false, true];
    keccak_c::<13>(m, &suffix, 384)
}
pub fn sha3_512(m:&Vec<u8>) -> Vec<u8> {
    let suffix = vec![false, true];
    keccak_c::<9>(m, &suffix, 512)
}

pub fn shake128(m:&Vec<u8>, output_length:usize) -> Vec<u8> {
    let suffix = vec![true;4];
    keccak_c::<21>(m, &suffix, output_length)
}

pub fn shake256(m:&Vec<u8>, output_length:usize) -> Vec<u8> {
    let suffix = vec![true;4];
    keccak_c::<17>(m, &suffix, output_length)
}

pub mod unofficial_sha {
    use super::*;
    /// from the designs of shake128 and shake256 who have a hidden state of 256 and 512 bits respectively a logical continuation is shake512 with a hidden state of 1024 bits
    /// It is very important to note that this is not an official hash function, it's security has not been proven.
    pub fn shake512(m:&Vec<u8>, output_length:usize) -> Vec<u8> {
        let suffix = vec![true;4];
        keccak_c::<9>(m, &suffix, output_length)
    }
}






