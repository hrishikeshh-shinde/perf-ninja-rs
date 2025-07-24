baseline:
time:   [207.96 µs 213.96 µs 222.96 µs]

new struct layout: reduces compiler-inserted padding layout
time:   [158.62 µs 158.68 µs 158.75 µs]
change: [-35.621% -29.801% -24.921%] (p = 0.00 < 0.05)
Performance has improved.

f64 to f32 didnt improve the performance but degraded it.

Theory:

Padding:
-> 32-bit architecture reads 4 bytes in a single CPU cycle. 
-> Structure padding is defined as the process of adding one or more empty bytes between the different data types to align data in memory. Structure padding increases memory consumption but is reduces CPU cycles. Structure contains structure members which can be accessed by a processor in chunks of 4 bytes at a time.

Bit-Packing:
Data packing is beneficial in places where additional computation is cheaper than the delay caused by inefficient memory transfers.

