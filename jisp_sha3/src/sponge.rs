use crate::keccak::{keccak, print_state_string};
use crate::preprocessing::{padding, split_bytes};


/// Performs the keccak[c] algorithm 
/// ```
/// ```
pub fn keccak_c<const RATE:usize>(m:&Vec<u8>, suffix:&Vec<bool>, output:usize) -> Vec<u8> {
    let blocks = padding::<RATE>(m, suffix);
    let rounds = 24; //rounds per block
    //absorb blocks
    let mut state = [0u64; 25];

    let mut k = 0;
    for block in blocks {
        //absorption
        for i in 0..RATE {
            state[i] ^= block[i];
        }
        print_state_string(format!("Xor'd: {}", k), &state);
        state = keccak(state, rounds);

        //print for keeping progress
        k += 1;
        print_state_string(format!("Permuted: {}", k), &state);
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

#[cfg(test)]
mod tests {
    use crate::keccak::print_state_u8;

    use super::*;

    #[test]
    fn keccak_224() {
 
        let m = vec![];
        let suffix = vec![false, true];
        let res = keccak_c::<18>(&m, &suffix, 224);
 
        print_state_u8("Hash".to_owned(), &res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn keccak_224_heavy() {
 
        let m = vec![0xc5u8; 200];
        let suffix = vec![false, true];
        let res = keccak_c::<18>(&m, &suffix, 224);
 
        print_state_u8("Hash".to_owned(), &res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn keccak_512() {
 
        let m = vec![];
        let suffix = vec![false, true];
        let res = keccak_c::<9>(&m, &suffix, 512);
 
        print_state_u8("Hash".to_owned(), &res);
        assert_eq!(res, vec![]);
    }
}