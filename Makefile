test:
	cargo test --release

bench:
	cargo bench

ignored:
	cargo test --release -- --ignored --nocapture

.PHONY: test bench ignored
