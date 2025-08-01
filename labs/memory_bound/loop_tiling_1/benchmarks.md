### Optimization: Loop Tiling for Matrix Transpose

- **What changed:**  
    Applied loop tiling (blocking) with `tile_size = 16` to reduce cache misses.

- **Why:**  
    Improved temporal and spatial locality
    Reduced cache conflict misses
    Exploits data reuse within small blocks

- **Performance Gain:**  
Baseline
time:   [10.597 ms 11.118 ms 11.734 ms]

Optimized
time:   [5.5716 ms 5.6472 ms 5.7331 ms]
change: [-51.967% -49.206% -46.633%] (p = 0.00 < 0.05)
Performance has improved.