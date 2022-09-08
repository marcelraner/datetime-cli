.PHONY: build run

build:
	cargo fmt
	cargo build

run: build
	cargo run

test: build
	cargo test -- --nocapture