use std::ops::{BitXor, BitAnd};

use crate::state::{State, Sheet, X, Y, Z,State_iter, to_state, from_state};
use crate::preprocessing::{split_bytes,flip_ordering};

pub fn keccak(str_state:[u64;25], rounds:i64) -> [u64;25] {
    let mut state = to_state(&str_state.to_vec());
    let n = 24;
    for i in (n - rounds)..n {
        state = round(state, i);
    }

    return from_state(&state);
}

pub fn round(state:State, round_index:i64 ) -> State {
    iota(chi(pi(theta(state))),round_index)
}


pub fn theta(state: State) -> State {
    //we are abusing notation here since sheets usually have a constant x while they now have a constant y
    let mut C = Sheet::default();
    let mut D = Sheet::default();
    let mut A = State::default();
    
    // Fill C
    for (x,_,z) in State_iter::xz() {
        let sum = xor_sum(&(0..5).map(|i| state[x][Y::from(i)].get(z)).collect());
        C[x].set(z,sum);
    }

    // Fill D
    for (x,_,z) in State_iter::xz() {
        let a = C[x - 1].get(z);
        let b = C[x + 1].get(z -1);

        D[x].set(z, a.bitxor(b));
    }

    // Fill Result
    for (x,y,z) in State_iter::new() {
        let a_xyz = state[x][y].get(z).bitxor(D[x].get(z));
        A[x][y].set(z, a_xyz);
    }

    return A;
}

pub fn rho(state:State) -> State {
    let mut A = State::default();
    let zero = X::from(0);
    A[zero][zero] = state[zero][zero];

    let (mut x, mut y) = (X::from(1),Y::from(0));
    for t in 0..=23 {
        let shift = Z::from(((t + 1) * (t + 2))/2);
        for z in (0..64).map(Z::from) {
            let modz = z - shift;
            A[x][y].set(z, state[x][y].get(modz));
        }
        (x, y) = (y, x*2 + y*3);
    }
    return A;
}

pub fn pi(state:State) -> State {
    let mut A = State::default();
    for (x,y,z) in State_iter::new() {
        A[x][y].set(z, state[x + y*3][x].get(z));
    }
    return A;
}

pub fn chi(state:State) -> State {
    let mut A = State::default();
    for (x,y,_) in State_iter::xy() {
        let bit = (state[x + 1][y].0 ^u64::MAX) & state[x + 2][y].0;
        let bit = state[x][y].0 ^bit;
        A[x][y].0 = bit;
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
        let (x,y) = (X::from(0), Y::from(0));
        let res = RC[z];
        let z = Z::from(z as i64);
        let res = A[x][y].get(z) ^ res;
        A[x][y].set(z, res);
    }

    return A
}

pub fn rc(t:i64) -> u8 {
    let mut R = 1;

    if modulus(t,255) == 0 {return 1;}

    for i in 1..=modulus(t,255) {
        //append 0
        R = R << 1;

        
        let bit8 = R >>8 & 1;
        println!("t:{}, bit:{}", t, bit8);
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


fn print_state(s:String, state:&State) {
    let v = flip_ordering(&split_bytes(&from_state(state).to_vec()));
    print!("{}: [\n", s);
    for elem in v {
        print!("{:02x?}, ", elem);
    }
    println!("];");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theta_basic() {
        let mut v = vec![0u64;18];
        v[0] = 0b0110 << 60;
        v[17] = 1;
        let mut state = to_state(&v);
        state = theta(state);
        print_state(format!("theta"), &state);
        state = rho(state);
        print_state(format!("rho"), &state);
        state = pi(state);
        print_state(format!("pi"), &state);
        state = chi(state);
        print_state(format!("chi"), &state);
        state = iota(state, 0);
        print_state(format!("iota"), &state);
        assert_eq!(state.0[0].0[0].0, 0);
    }

    #[test]
    fn rc_test() {
        let result = rc(0);
        assert_eq!(result, 1);

        let result = rc(1);
        assert_eq!(result, 0);
    }
}