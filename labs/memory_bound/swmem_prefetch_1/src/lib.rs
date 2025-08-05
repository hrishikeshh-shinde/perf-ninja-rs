#[cfg(test)]
mod tests;

const _MM_HINT_T0: i32 = 3;

pub const HASH_MAP_SIZE: usize = 32 * 1024 * 1024 - 5; //hash map conatins around 32M Integers
const NUMBER_OF_LOOKUPS: usize = 1024 * 1024;

const UNUSED: i32 = i32::MAX;
pub struct HashMapT {
    m_vector: Vec<i32>,
    n_buckets: usize,
}
impl HashMapT {
    pub fn new(size: usize) -> HashMapT {
        HashMapT {
            m_vector: vec![UNUSED; size],
            n_buckets: size,
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        let bucket: usize = val as usize % self.n_buckets;
        if self.m_vector[bucket] == UNUSED {
            self.m_vector[bucket] = val;
            true
        } else {
            false
        }
    }

    pub fn find(&self, val: i32) -> bool {
        let bucket = val as usize % self.n_buckets;
        self.m_vector[bucket] != UNUSED
    }
}

fn get_sum_of_digits(mut n: i32) -> i32 {
    let mut sum = 0;
    while n != 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

// Prefetch function for x86_64
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
#[inline]
pub unsafe fn prefetch<T>(addr: *const T) {
    use core::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};
    _mm_prefetch(addr as *const i8, _MM_HINT_T0);
}

// Dummy fallback for other platforms
#[cfg(not(all(target_arch = "x86_64", target_feature = "sse")))]
#[inline]
pub unsafe fn prefetch<T>(_addr: *const T) {
    // No-op on unsupported platforms
}

pub fn solution(hash_map: &HashMapT, lookups: &[i32]) -> i32 {
    let mut result = 0;
    let prefetch_distance = 8;


    for i in 0..lookups.len() {
        if i + prefetch_distance < lookups.len() {
            // Software prefetch to T0 cache
            unsafe {
                let future = &lookups[i + prefetch_distance];
                prefetch(future as *const _);
            }
        }

        let val = lookups[i];
        if hash_map.find(val) {
            result += get_sum_of_digits(val);
        }
    }

    // for &val in lookups { //we know the values in advance
    //     if hash_map.find(val) { //most cache misses happen here
    //         result += get_sum_of_digits(val);
    //     }
    // }

    result
}


pub fn init(hash_map: &mut HashMapT) -> Vec<i32> {
    use rand::prelude::*;
    let mut generator = thread_rng();

    for _ in 0..HASH_MAP_SIZE {
        hash_map.insert(generator.gen());
    }

    let mut lookups = Vec::with_capacity(NUMBER_OF_LOOKUPS);
    lookups.resize_with(NUMBER_OF_LOOKUPS, || generator.gen());

    lookups
}
