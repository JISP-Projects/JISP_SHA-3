use std::ops::{Index, IndexMut, BitXor};
pub struct Lane(u64);
pub struct Sheet([Lane;5]);
pub struct State([Sheet;5]);

impl Index<usize> for Lane {
    type Output = u8;

    fn index(&self, index:usize) -> &Self::Output {
        let Lane(num) = self;
        let res = (num >> index) % 2;
        return &(res as u8);
    }
}

impl Lane {
    fn set(&mut self, index:usize, value:bool) {
        if self[index] != value as u8 {
            let Lane(num) = self;
            *num = num.bitxor(1 << index);
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


