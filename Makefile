.PHONY: test run

test:
	cargo fmt
	cargo clippy
	cargo test

run: test
	cargo run
