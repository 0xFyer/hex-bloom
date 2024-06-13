# HexBloom

A simple implementation of the LinkedHashX data structure introduced in [HexBloom](https://eprint.iacr.org/2021/773).

## Benchmark

```bash
$ cargo run
    Compiling hex-bloom v0.1.0 (/home/src/github.com/0xfyer/hex-bloom)
     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/hex-bloom`
Time taken for 500000 insertions: 3.392742698s
Time taken for 500000 updates: 4.472658695s
Time taken for 500000 deletions: 216.692728ms
Time taken for 1000000 insertions: 6.763830098s
Time taken for 1000000 updates: 8.924935579s
Time taken for 1000000 deletions: 432.109456ms
Time taken for 2000000 insertions: 13.483849306s
Time taken for 2000000 updates: 17.887658309s
Time taken for 2000000 deletions: 864.881887ms
Time taken for 3000000 insertions: 20.258414294s
Time taken for 3000000 updates: 28.030640787s
Time taken for 3000000 deletions: 1.510717129s
```

