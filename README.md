# playing

This repository is a scratchspace of mine to play around with some Rust functionality (and measure its impact on runtime). Note that none of this code is intended to be used in production somewhere. 

It currently tests different ways to sum up the entries of a 1024x1024 matrix of type ```Vec<Vec<f32>>``` which represents a collection of row-vectors. It has three different approaches implemented in [lib.rs](src/lib.rs): column-wise summation (which has very bad cache locality), row-wise summation (much better cache locality) and SIMD-accelerated row-wise summation. Interestingly there is a factor of 10x difference on the runtime (on my machine): 

```
$ cargo bench

running 3 tests
test bench_colwise_summation            ... bench:   8,297,295 ns/iter (+/- 886,357)
test bench_rowwise_simd_summation_sse41 ... bench:     851,027 ns/iter (+/- 28,224)
test bench_rowwise_summation            ... bench:   3,377,313 ns/iter (+/- 180,663)
```

If we install [cargo-profiler](https://github.com/kernelmachine/cargo-profiler), we can get some statistics about the cache misses incurred by our different implementations. The numbers confirm what we expected: the column-wise access pattern results in 8 times more L1 cache misses:

```
$ cargo profiler cachegrind --bin ./target/release/colwise

Profiling colwise with cachegrind...

Total Memory Accesses...46,824,375	

Total L1 I-Cache Misses...332 (0%)	
Total LL I-Cache Misses...260 (0%)	
Total L1 D-Cache Misses...1,128,715 (2%)	
Total LL D-Cache Misses...68,207 (0%)	
```

```
$ cargo profiler cachegrind --bin ./target/release/rowwise

Profiling rowwise with cachegrind...

Total Memory Accesses...40,419,121	

Total L1 I-Cache Misses...372 (0%)	
Total LL I-Cache Misses...298 (0%)	
Total L1 D-Cache Misses...140,739 (0%)	
Total LL D-Cache Misses...69,659 (0%)	
```
