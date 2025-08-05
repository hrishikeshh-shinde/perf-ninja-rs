#![feature(maybe_uninit_array_assume_init)]

use rand::prelude::*;

use std::cmp::{Ord, Ordering, PartialOrd};

#[cfg(test)]
mod tests;

mod init;
pub use init::{create_entry, init};

// Assume those constants never change
pub const N: usize = 10_000;
pub const MIN_RANDOM: i32 = 0;
pub const MAX_RANDOM: i32 = 100;

// FIXME: this data structure can be reduced in size
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct S {
    pub d: f32, //4 bytes
    pub l: i16, //max-value = 10^4 so 2 bytes are enough
    pub i: u8, //max-value = 100 so 1 byte enough
    pub s: u8, //max-value = 100 so 1 byte enough
    pub b: bool, //1byte
}

// C++ version overloads '<' operator like this:
//   bool operator<(const S &s) const { return this->i < s.i; }
// I presume that means we order / sort / compare based only on value of 'i'?

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.i.partial_cmp(&other.i)
    }
}
impl Ord for S {
    fn cmp(&self, other: &Self) -> Ordering {
        self.i.cmp(&other.i)
    }
}
impl PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}
impl Eq for S {}

pub fn solution(arr: &mut [S]) {
    // use std::mem::{size_of, align_of};
    // println!("Size of S: {}", size_of::<S>());
    // println!("Align of S: {}", align_of::<S>());
    

    // 1. shuffle
    let mut rd = thread_rng();
    arr.shuffle(&mut rd);

    // 2. sort
    arr.sort_unstable();
}
