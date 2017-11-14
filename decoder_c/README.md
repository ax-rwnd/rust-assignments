# Stack Allocated
```
   Compiling decoder v0.1.0 (file:///home/axel/code/rust-assignments/decoder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59 secs
     Running `target/debug/decoder`
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', src/main.rs:25:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

shell returned 101
```

# Heap Allocated
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `/home/axel/code/rust-assignments/decoder_b/target/debug/decoder`
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', main.rs:30:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

shell returned 101
```
# What happens if the null terminators are removed?
The same thing; By the rust memory model, mutable slices are borrowed from the two arrays while removing one for each recursion.
