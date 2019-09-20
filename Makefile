test:
	@echo ~~~~~~~~~~~~~~~ Running rust implementation unit tests ~~~~~~~~~~~~~~~
	cargo test --release \
		test_

test_against_python:
	@echo ~~~~~~~~~~~~~~~ Running fuzz tests against python ~~~~~~~~~~~~~~~
	cargo test --release \
		test_py_ \
		-- --ignored --nocapture
	cargo test --release \
		test_implementations_are_equivalent \
		-- --ignored

test_eip_152_vec_8:
	@echo ~~~~~~~~~~~~~~~ Running slow EIP 152 test vector 8 ~~~~~~~~~~~~~~~
	cargo test --release \
		test_blake2b_compress_eip_152_vec_8 \
		-- --ignored --nocapture

bench:
	@echo ~~~~~~~~~~~~~~~ Running rust implementation benchmarks ~~~~~~~~~~~~~~~
	cargo bench

test_short: test test_against_python
test_all: test test_against_python bench test_eip_152_vec_8

clean:
	rm -rf *.egg-info build dist target pip-wheel-metadata

.PHONY: test test_against_python test_eip_152_vec_8 bench test_short test_all clean
