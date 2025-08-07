#![feature(thread_id_value)]

#[cfg(test)]
mod tests;

use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

pub fn solution(data: &[u32], thread_count: usize) -> i32 {
    // Using std::atomic counters to disallow compiler to promote `target`
    // memory location into a register. This way we ensure that the store
    // to `target` stays inside the loop.
    #[repr(align(64))]
    struct AlignedAtomicU32 {
        value: AtomicU32
    }

    let mut accumulators = Vec::with_capacity(thread_count);
    accumulators.resize_with(thread_count, || AlignedAtomicU32{ value: AtomicU32::new(0) });
    let chunks = data.chunks(data.len() / thread_count);
    thread::scope(|s| {
        // C++ uses `#pragma omp for` which splits into chunks, just like this
        for (idx, chunk) in chunks.enumerate() {
            let target_acc = &accumulators[idx % thread_count].value;
            s.spawn(move || {
                for v in chunk {
                    // Perform computation on each input
                    let mut item = *v;
                    item += 1000;
                    item ^= 0xADEDAE;
                    item |= item >> 24;

                    // Write result to accumulator
                    target_acc.fetch_add(item % 13, Ordering::SeqCst);
                }
            });
        }
    });

    accumulators
        .iter()
        .map(|v| v.value.load(Ordering::SeqCst) as usize)
        .sum::<usize>()
        .try_into()
        .unwrap()
}
