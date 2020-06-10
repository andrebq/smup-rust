.PHONY: test update build doc run fmt run-fast

update:
	rustup update

test: build
	cargo test

build:
	cargo build

doc:
	cargo doc

run:
	cargo run

run-fast:
	cargo run --release

fmt:
	cargo fmt
