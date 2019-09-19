test:
	@echo ~~~~~~~~~~~~~~~ Running rust implementation unit tests ~~~~~~~~~~~~~~~
	cargo test --release \
		test_

bench:
	@echo ~~~~~~~~~~~~~~~ Running rust implementation benchmarks ~~~~~~~~~~~~~~~
	cargo bench

test_eip_152_vec_8:
	@echo ~~~~~~~~~~~~~~~ Running slow EIP 152 test vector 8 ~~~~~~~~~~~~~~~
	cargo test --release \
		test_blake2b_compress_eip_152_vec_8 \
		-- --ignored --nocapture

test_against_python:
	@echo ~~~~~~~~~~~~~~~ Running fuzz tests against python ~~~~~~~~~~~~~~~
	cargo test --release \
		test_py_ \
		-- --ignored --nocapture
	cargo test --release \
		test_implementations_are_equivalent \
		-- --ignored

test_all: test test_against_python bench test_eip_152_vec_8

.PHONY: test bench test_eip_152_vec_8 test_against_python test_all
