.PHONY: build check clippy fmt test all

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

all: fmt check clippy build test
