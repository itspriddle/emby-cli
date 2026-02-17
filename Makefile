.PHONY: build release test check fix fmt clippy clean

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

check: fmt-check clippy

fix: fmt-fix clippy-fix

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

fmt-fix:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

clippy-fix:
	cargo clippy --fix --allow-dirty -- -D warnings

clean:
	cargo clean
