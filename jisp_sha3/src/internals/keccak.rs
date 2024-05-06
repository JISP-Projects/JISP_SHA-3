//! The internals of the Keccak permutation function
#![allow(non_snake_case)]
use std::ops::BitXor;

use crate::internals::state::{State, Sheet, to_state, from_state, Modulus};

pub fn keccak(str_state:[u64;25], rounds:i64) -> [u64;25] {
    let mut state = to_state(&str_state.to_vec());
    let n = 24;
    for i in (n - rounds)..n {
        state = round(state, i);
    }

    return from_state(&state);
}

pub fn round(state:State, round_index:i64 ) -> State {
    return iota(chi(pi(rho(theta(state)))),round_index);
}


pub fn theta(state: State) -> State {
    //we are abusing notation here since sheets usually have a constant x while they now have a constant y
    let mut C = Sheet::default();
    let mut D = Sheet::default();
    let mut A = State::default();
    
    // Fill C
    for x in 0..5 {
        for z in 0..64 {
            let sum = xor_sum(&(0..5).map(|i| state[x][i].get(z)).collect());
            C[x].set(z,sum);
        }
    }

    // Fill D
    for x in 0..5 {
        for z in 0..64 {
            let a = C[x - 1].get(z);
            let b = C[x + 1].get(z -1);

            D[x].set(z, a ^ b);
        }
    }

    // Fill Result
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..64 {
                let a_xyz = state[x][y].get(z) ^ D[x].get(z);
                A[x][y].set(z, a_xyz);
            }
        }
    }
    return A;
}

pub fn rho(state:State) -> State {
    let mut A = State::default();
    A[0][0] = state[0][0];

    let (mut x, mut y) = (1, 0);
    for t in 0..=23 {
        let shift = (((t + 1) * (t + 2))/2).md(64);
        for z in 0..64 {
            let modz:i64 = z - shift;
            A[x][y].set(z, state[x][y].get(modz));
        }
        (x, y) = (y, (x*2 + y*3).md(5));
    }
    return A;
}

pub fn pi(state:State) -> State {
    let mut A = State::default();
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..64 {
                A[x][y].set(z, state[x + y*3][x].get(z));
            }
        }
    }
    return A;
}

pub fn chi(state:State) -> State {
    let mut A = State::default();
    for x in 0..5 {
        for y in 0..5 {
            let bit = (state[x + 1][y].0 ^u64::MAX) & state[x + 2][y].0;
            let bit = state[x][y].0 ^bit;
            A[x][y].0 = bit;
        }
    }
    return A;
}

pub fn iota(state:State, round_index:i64) -> State {
    let mut A = state;
    let mut RC = [0;64];

    for j in 0..=6 {
        RC[(1 << j) - 1] = rc(j + 7*round_index);
    }

    for z in 0..64 {
        let (x,y) = (0, 0);
        let res = A[x][y].get(z) ^ RC[z as usize];
        A[x][y].set(z, res);
    }

    return A
}

pub fn rc(t:i64) -> u8 {
    let mut R = 1;

    if modulus(t,255) == 0 {return 1;}

    for _ in 1..=modulus(t,255) {
        //append 0
        R = R << 1;

        
        let bit8 = R >>8 & 1;
        R ^= bit8;
        R ^= bit8 << 4;
        R ^= bit8 << 5;
        R ^= bit8 << 6;

        //truncate
        R &= 0xFF; 
    }
    return (R & 1) as u8
}

//utility functions

fn xor_sum(v: &Vec<u8>) -> u8 {
    let mut res = 0;
    for bit in v {
        res = res.bitxor(bit);
    }
    return res;
}

fn modulus(x:i64, m:u8) -> u8 {
    let m = m as i64;
    let mut x = x;
    while x < 0 {
        x += m;
    }
    while x >= m {
        x -= m;
    }
    return x as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_test() {
        let result = rc(0);
        assert_eq!(result, 1);

        let result = rc(1);
        assert_eq!(result, 0);
    }
}