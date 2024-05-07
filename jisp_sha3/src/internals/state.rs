//! The internal state in the keccak algorithm
use std::ops::{Index, IndexMut, BitXor};

/// Turns a string of `u64` words into a state matrix
/// 
/// # Panics
/// 
/// If the string of words has more than 25 elements
/// 
/// # Examples
/// 
/// ```
/// use jisp_sha3::internals::state::to_state;
/// 
/// let v = vec![0,0,0,0,1,2];
/// let state = to_state(&v);
/// 
/// assert_eq!(state[4][0].0, 1);
/// assert_eq!(state[0][1].0, 2);
/// ```
pub fn to_state(v:&Vec<u64>) -> State {
    let mut res = State::default();
    let mut x = 0;
    let mut y = 0;

    for word in v {
        res[x][y] = Lane(*word);
        x = x + 1;
        if x >= 5 {
            y = y + 1;
            x = 0;
        }
    }

    return res;
}

/// Transforms a State back into a string of `u64` words
/// 
/// # Examples
/// ```
/// use jisp_sha3::internals::state::{to_state, from_state};
/// 
/// let mut v = [0;25];
/// v[4] = 1;
/// v[5] = 2;
/// 
/// let state = to_state(&v.to_vec());
/// let v2 = from_state(&state);
/// 
/// assert_eq!(v,v2);
/// ```
pub fn from_state(state:&State) -> [u64;25] {
    let mut res = [0;25];

    for x in 0..5 {
        for y in 0..5 {
            let i = (x + y*5) as usize;
            res[i] = state[x][y].0;
        }
    }
    return res;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Lane(pub u64);

#[derive(Debug, Default, Clone, Copy)]
pub struct Sheet(pub [Lane;5]);

#[derive(Debug, Default, Clone, Copy)]
pub struct State(pub [Sheet;5]);

pub trait Modulus {
    fn md(&self, m:usize) -> Self;
}

impl Modulus for i64 {
    fn md(&self, m:usize) -> i64 {
        let m = m as i64;
        let mut x = *self;
        while x < 0 {
            x += m;
        }
        while x >= m {
            x -= m;
        }
        return x;
    }
}

impl Lane {

    pub fn get(&self, index:i64) -> u8 {
        let num = self.0;
        let res = (num >> (63 - index.md(64))) % 2;
        return res as u8;
    }

    pub fn set(&mut self, index:i64, value:u8) {
        if self.get(index) != value {

            let num = &mut self.0;
            *num = num.bitxor(1 << (63 - index.md(64)));
        }
    }
}


impl Index<i64> for Sheet {
    type Output = Lane;

    fn index(&self, index: i64) -> &Self::Output {
        let Sheet(data) = self;
        &data[index.md(5) as usize]
    }
}

impl IndexMut<i64> for Sheet {
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        let Sheet(data) = self;
        &mut data[index.md(5) as usize]
    }
}

impl Index<i64> for State {
    type Output = Sheet;

    fn index(&self, index: i64) -> &Self::Output {
        let State(data) = self;
        &data[index.md(5) as usize]
    }
}

impl IndexMut<i64> for State {
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        let State(data) = self;
        &mut data[index.md(5) as usize]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_get() {
        let lane = Lane(0b0010);
        assert_eq!(lane.get(63), 0);
        assert_eq!(lane.get(62), 1);
    }

    #[test]
    fn lane_set() {
        let mut lane = Lane(1 << 63);
        lane.set(0, 1);
        assert_eq!(lane.0, 1 << 63);
        lane.set(1, 1);
        lane.set(0, 0);
        assert_eq!(lane.0, 1 << 62);
    }
}