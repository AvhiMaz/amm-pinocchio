.PHONY: build check clippy fmt test bench all

build:
	cargo build

check:
	cargo check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt; cargo +nightly fmt --all

test:
	cargo test -- --nocapture

bench:
	cargo build-sbf && cargo bench --bench initializer_ix_bench

all: fmt check clippy build test bench
