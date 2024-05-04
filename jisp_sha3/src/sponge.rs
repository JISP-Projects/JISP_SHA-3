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
/// use jisp_sha3::sponge::to_state;
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
        x += 1;
        if x >= 5 {
            y += 1;
            x = 0;
        }
    }

    return res;
}

/// Transforms a State back into a string of `u64` words
/// 
/// # Examples
/// ```
/// use jisp_sha3::sponge::{to_state, from_state};
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
            res[x + 5*y] = state[x][y].0;
        }
    }

    return res;
}

#[derive(Debug, Default)]
pub struct Lane(pub u64);

#[derive(Debug, Default)]
pub struct Sheet(pub [Lane;5]);

#[derive(Debug, Default)]
pub struct State(pub [Sheet;5]);


impl Lane {

    pub fn get(&self, index:usize) -> u8 {
        let num = self.0;
        let res = (num >> (63 - index)) % 2;
        return res as u8;
    }

    pub fn set(&mut self, index:usize, value:u8) {
        if self.get(index) != value {
            let num = &mut self.0;
            *num = num.bitxor(1 << (63 - index));
        }
    }
}


impl Index<usize> for Sheet {
    type Output = Lane;

    fn index(&self, index: usize) -> &Self::Output {
        let Sheet(data) = self;
        &data[index]
    }
}

impl IndexMut<usize> for Sheet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let Sheet(data) = self;
        &mut data[index]
    }
}

impl Index<usize> for State {
    type Output = Sheet;

    fn index(&self, index: usize) -> &Self::Output {
        let State(data) = self;
        &data[index]
    }
}

impl IndexMut<usize> for State {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let State(data) = self;
        &mut data[index]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_get() {
        let mut lane = Lane(0b0010);
        assert_eq!(lane.get(63), 0);
        assert_eq!(lane.get(62), 1);
    }

    #[test]
    fn lane_set() {
        let mut lane = Lane(1 << 63);
        lane.set(0,1);
        assert_eq!(lane.0, 1 << 63);
        lane.set(1,1);
        lane.set(0,0);
        assert_eq!(lane.0, 1 << 62);
    }
}