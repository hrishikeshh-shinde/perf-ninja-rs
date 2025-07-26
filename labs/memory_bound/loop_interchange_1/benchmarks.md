## Matrix Power Optimization (Loop Interchange 1)

### Baseline Performance

time: [726.04 ms 726.96 ms 728.20 ms]

### Optimized Performance

time: [82.262 ms 83.716 ms 85.316 ms]
change: [-88.657% -88.484% -88.255%] (p = 0.00 < 0.05)

### Notes

- Applied **loop interchange** in the matrix multiplication function.
- Improved **cache locality** and reduced memory traffic.
- Achieved a **~88% performance improvement**.
- Indicates the original code was likely **memory bound**.