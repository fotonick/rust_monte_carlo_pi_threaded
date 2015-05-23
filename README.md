Multi-threaded Monte-Carlo estimation of π
==========================================

This code extends the Monte Carlo estimation of π example from the [rand module
documentation](http://doc.rust-lang.org/rand/rand/index.html#monte-carlo-estimation-of-π).
It spawns multiple threads to run trials, then combines the results. This work
was performed as a learning exercise, so the code may not be ideal or idiomatic.
Improvements are welcome.
