.PHONY: build check clippy fmt test bench all

build:
	cargo build-sbf

check:
	cargo check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt; cargo +nightly fmt --all

test:
	cargo test 

bench:
	cargo bench --bench initializer_ix_bench && cargo bench --bench add_liquidity_ix_bench && cargo bench --bench swap_ix_bench && cargo bench --bench withdraw_ix_bench

all: fmt check clippy build test bench
