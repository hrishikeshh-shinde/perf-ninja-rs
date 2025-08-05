# My Solutions to Performance Ninja Labs (Rust)

This repository contains **my personal solutions** to the Performance Ninja labs, ported to Rust from the original C++ repository:  
🔗 [Original Repo](https://github.com/dendibakh/perf-ninja)

---

I've been really enjoying these tasks — they're challenging, rewarding, and teach you a lot about how modern CPUs actually behave. I forked the repo just to keep a personal copy and track my own progress and improvements. I had real fun solving these!

---

## Labs Completed So Far

These are grouped by category. I've implemented and benchmarked all of these locally:

### ✅ Core Bound:

### ✅ Memory Bound:
- Data Packing
- Loop Interchange 1
- Loop Interchange 2
- Loop Tiling
- Software Memory Prefetching

### ✅ Bad Speculation:


### ✅ Misc:
- Warmup

---

## How I Run & Test

I benchmark with `cargo bench` and validate correctness with `cargo test`.  
I've also tried using tools like `samply` and `pprof-rs` for deeper performance analysis.

---

## Final Thoughts

The labs are a fantastic way to go beyond “writing code that works” and start thinking about **how code runs**.  
The insights I’ve gained are directly applicable to building high-performance systems in Rust and beyond.

Big thanks to [Denis Bakhvalov](https://easyperf.net) for the original project and [Graham King](https://github.com/grahamking) for the Rust port.

If you're working through these labs too — let’s connect!

---

## License

The original work is ©️ 2021 by Denis Bakhvalov, licensed under CC BY 4.0.  
This fork includes modifications and solutions by me (Hrishikesh Shinde), built on top of the Rust port by Graham King.
