# Blake2

Blake2 compression in Rust with Python bindings.

## Running the tests

The tests must be run with a release binary.  Rust panics on integer overflow
in debug binaries which prevents our implementation from functioning.  To run
the tests in release mode, run this command:
```bash
cargo test --release
```

### Benchmarking calculations with higher rounds

Some benchmarks are included for hash calculations using 2 million and 8
million rounds.  To run them, run this command:
```bash
cargo bench
```

### Running (and benchmarking) the slow 8th test vector from EIP 152

The test covering test vector 8 from EIP 152
(https://eips.ethereum.org/EIPS/eip-152#test-vector-8) takes a little while to
run.  Because of this, it's ignored in the normal test suite.  To run the test
and see a message describing how long it took, run this command:
```bash
cargo test --release -- --ignored --nocapture
```
