.PHONY: test
test: test_rust test_python

.PHONY: test_rust
test_rust:
	@echo ~~~~~~~~~~~~~~~ Running rust implementation unit tests ~~~~~~~~~~~~~~~
	cargo test test_

.PHONY: test_python
test_python:
	@echo ~~~~~~~~~~~~~~~ Running python binding tests ~~~~~~~~~~~~~~~
	tox

.PHONY: bench
bench:
	@echo ~~~~~~~~~~~~~~~ Running rust implementation benchmarks ~~~~~~~~~~~~~~~
	cargo bench

.PHONY: test_rust_eip_152_vec_8
test_rust_eip_152_vec_8:
	@echo ~~~~~~~~~~~~~~~ Running slow EIP 152 test vector 8 ~~~~~~~~~~~~~~~
	cargo test --release \
		test_f_compress_eip_152_vec_8 \
		-- --ignored --nocapture

.PHONY: test_all
test_all: test_rust test_python bench test_rust_eip_152_vec_8

.PHONY: clean
clean:
	rm -rf *.egg-info build dist target pip-wheel-metadata

CURRENT_SIGN_SETTING := $(shell git config commit.gpgSign)
.PHONY: bumpversion
bumpversion:
	git config commit.gpgSign true
	bumpversion $(bump)
	git config commit.gpgSign "$(CURRENT_SIGN_SETTING)"
	git push upstream && git push upstream --tags

.PHONY: collectwheels
collectwheels:
	GITHUB_PROJECT=davesque/blake2b-py \
		CIRCLE_JOB_NAME=linux-bdist-and-sdist \
		OUTPUT_DIR=target/wheels \
		./ci/collectwheels.py

.PHONY: publish
publish:
	twine upload target/wheels/*
