use std::{ops::{Index, IndexMut, BitXor, Add, Sub, Mul}, iter};

//idea: use structs for x, y and z that index the sets and immediately allow for the correct mod operations
//add iterators for constant x, constant y and constant z.

/// Turns a string of `u64` words into a state matrix
/// 
/// # Panics
/// 
/// If the string of words has more than 25 elements
/// 
/// # Examples
/// 
/// ```
/// use jisp_sha3::state::to_state;
/// 
/// let v = vec![0,0,0,0,1,2];
/// let state = to_state(&v);
/// 
/// assert_eq!(state[4][0].0, 1);
/// assert_eq!(state[0][1].0, 2);
/// ```
pub fn to_state(v:&Vec<u64>) -> State {
    let mut res = State::default();
    let mut x = X::from(0);
    let mut y = Y::from(0);

    for word in v {
        res[x][y] = Lane(*word);
        x = x +1;
        if x.0 == 0 {
            y = y + 1;
        }
    }

    return res;
}

/// Transforms a State back into a string of `u64` words
/// 
/// # Examples
/// ```
/// use jisp_sha3::state::{to_state, from_state};
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

    for (x,y,_) in State_iter::xy() {
        let i = x.size() + y.size()*5;
        res[i] = state[x][y].0;
    }
    return res;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Lane(pub u64);

#[derive(Debug, Default, Clone, Copy)]
pub struct Sheet(pub [Lane;5]);

#[derive(Debug, Default, Clone, Copy)]
pub struct State(pub [Sheet;5]);

#[derive(Debug, Clone, Copy)]
pub struct Coord<const MODULUS:usize>(i64);

pub type X = Coord<5>;
pub type Y = Coord<5>;
pub type Z = Coord<64>;

pub struct State_iter{
    x_mut:bool,
    y_mut:bool,
    z_mut:bool,
    x:X,
    y:Y,
    z:Z,
    done:bool
}

impl State_iter{
    pub fn new() -> State_iter {
        State_iter { 
            x_mut: true, 
            y_mut: true, 
            z_mut: true, 
            x: X::from(0), 
            y: Y::from(0), 
            z: Z::from(0),
            done : false
        }
    }
    pub fn xz() -> State_iter {
        State_iter { y_mut: false, ..Self::new() }
    }
    pub fn xy() -> State_iter {
        State_iter { z_mut: false, ..Self::new() }
    }
    pub fn yz() -> State_iter {
        State_iter { x_mut: false, ..Self::new() }
    }
}

impl Iterator for State_iter {
    type Item = (X,Y,Z);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let res = (self.x, self.y, self.z);
        let mut flipnext = true;
        if self.x_mut {
            self.x = self.x + 1;
            flipnext = self.x.0 == 0;
        }
        if self.y_mut && flipnext {
            self.y = self.y + 1;
            flipnext = self.y.0 == 0;
        }
        if self.z_mut && flipnext {
            self.z = self.z + 1;
            flipnext = self.z.0 == 0;
        }
        self.done = flipnext;
        return Some(res);
    }
}




impl<const MODULUS:usize> Coord::<MODULUS> {
    pub fn from<T:Into<i64>>(n:T) -> Coord<MODULUS> {
        let mut x = n.into();
        let m = MODULUS as i64;
        while x < 0 {
            x += m;
        }
        while x >= m {
            x -= m;
        }
        return Coord::<MODULUS>(x);
    }

    pub fn size(&self) -> usize {
        return self.0 as usize;
    }

}

impl<const MODULUS:usize> Add for Coord<MODULUS> {
    type Output = Coord<MODULUS>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
} 
impl<const MODULUS:usize> Add<usize> for Coord<MODULUS> {
    type Output = Coord<MODULUS>;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as i64)
    }
} 

impl<const MODULUS:usize> Sub for Coord<MODULUS> {
    type Output = Coord<MODULUS>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.0 - rhs.0)
    }
} 

impl<const MODULUS:usize> Sub<usize> for Coord<MODULUS> {
    type Output = Coord<MODULUS>;
    fn sub(self, rhs: usize) -> Self::Output {
        Self::from(self.0 - rhs as i64)
    }
} 

impl<const MODULUS:usize> Mul<usize> for Coord<MODULUS> {
    type Output = Coord<MODULUS>;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::from(self.0 * rhs as i64)
    }
}

impl Lane {

    pub fn get(&self, index:Z) -> u8 {
        let num = self.0;
        let res = (num >> (63 - index.0)) % 2;
        return res as u8;
    }

    pub fn set(&mut self, index:Z, value:u8) {
        if self.get(index) != value {

            let num = &mut self.0;
            *num = num.bitxor(1 << (63 - index.0));
        }
    }
}


impl Index<Y> for Sheet {
    type Output = Lane;

    fn index(&self, index: Y) -> &Self::Output {
        let Sheet(data) = self;
        &data[index.size()]
    }
}

impl IndexMut<Y> for Sheet {
    fn index_mut(&mut self, index: Y) -> &mut Self::Output {
        let Sheet(data) = self;
        &mut data[index.size()]
    }
}

impl Index<X> for State {
    type Output = Sheet;

    fn index(&self, index: X) -> &Self::Output {
        let State(data) = self;
        &data[index.size()]
    }
}

impl IndexMut<X> for State {
    fn index_mut(&mut self, index: X) -> &mut Self::Output {
        let State(data) = self;
        &mut data[index.size()]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_get() {
        let lane = Lane(0b0010);
        assert_eq!(lane.get(Z::from(63)), 0);
        assert_eq!(lane.get(Z::from(62)), 1);
    }

    #[test]
    fn lane_set() {
        let mut lane = Lane(1 << 63);
        lane.set(Z::from(0),1);
        assert_eq!(lane.0, 1 << 63);
        lane.set(Z::from(1),1);
        lane.set(Z::from(0),0);
        assert_eq!(lane.0, 1 << 62);
    }
}