##Rust benchmarking test
just a quick play project in rust to see which implementation is faster,
I'm comparing a classic filter map reduce (fold in rust's case) with
a classic loop and mutable counter, and a implementation with all of the logic
in the fold call, skipping the separate filter/map calls.

##results
on my system
```
     Running target/release/fold_test-8cf6e65aed375937

running 3 tests
test tests::bench_all_fold        ... bench:     570,619 ns/iter (+/- 28,234)
test tests::bench_raw_loop        ... bench:     576,963 ns/iter (+/- 17,125)
test tests::bench_sequential_fold ... bench:     744,286 ns/iter (+/- 35,980)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured
```

performance moves linearly and the logic-in-fold is seems to be the fastest
implementation, although my personal preference from coding style is
the sequential_fold.
