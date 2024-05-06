//! The internals of SHA-3 with freely adjustable parameters. Only use if you know what you are doing

use crate::internals::keccak::keccak;
use crate::preprocessing::{padding, split_bytes};


/// Performs the keccak[c] algorithm and provides an output of `output` bits long
pub fn keccak_c<const RATE:usize>(m:&Vec<u8>, suffix:&Vec<bool>, output:usize) -> Vec<u8> {
    let blocks = padding::<RATE>(m, suffix);
    let rounds = 24; //rounds per block
    //absorb blocks
    let mut state = [0u64; 25];

    for block in blocks {
        //absorption
        for i in 0..RATE {
            state[i] ^= block[i];
        }
        state = keccak(state, rounds);
    }


    //squeeze blocks
    let mut result = Vec::new();
    while result.len()*8 < output {
        // Extract truncated state
        let mut block = Vec::new();
        for i in 0..RATE {
            block.push(state[i]);
        }
        let block = split_bytes(&block);

        for word in block {
            result.push(word);
            if result.len()*8 >= output {
                break;
            }
        }

        if result.len()*8 < output {
            state = keccak(state, rounds);
        }
    }

    return result;
}