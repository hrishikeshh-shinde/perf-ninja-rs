## Performance
Baseline:
time:   [82.126 ms 86.655 ms 91.092 ms]

time:   [6.6176 ms 6.6947 ms 6.7970 ms]
change: [-92.666% -92.274% -91.831%] (p = 0.00 < 0.05)
Performance has improved.

## Changes Made
- Replaced `Vec<AtomicU32>` with a custom `#[repr(align(64))]` wrapper struct to pad each accumulator to a full cache line.
- This removed false sharing between threads writing to adjacent indices.
- Thread-local computations remained unchanged.


## Insights
- False sharing was occurring in the accumulator array, not the input data.
- Aligning each atomic variable to 64 bytes ensures each one resides in its own cache line.
- This eliminated cache line ping-ponging between threads and significantly boosted performance.

## Conclusion
Padding structures to avoid cache line sharing is **critical** in multi-threaded code that updates shared memory. This simple change led to a **10x+ speedup**.
