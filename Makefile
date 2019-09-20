test:
	@echo ~~~~~~~~~~~~~~~ Running rust implementation unit tests ~~~~~~~~~~~~~~~
	cargo test --release \
		test_

test_eip_152_vec_8:
	@echo ~~~~~~~~~~~~~~~ Running slow EIP 152 test vector 8 ~~~~~~~~~~~~~~~
	cargo test --release \
		test_blake2b_compress_eip_152_vec_8 \
		-- --ignored --nocapture

bench:
	@echo ~~~~~~~~~~~~~~~ Running rust implementation benchmarks ~~~~~~~~~~~~~~~
	cargo bench

test_all: test bench test_eip_152_vec_8

clean:
	rm -rf *.egg-info build dist target pip-wheel-metadata

.PHONY: test test_eip_152_vec_8 bench test_all clean
