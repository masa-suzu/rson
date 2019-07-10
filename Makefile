.PHONY: test run clean

test: clean
	cargo fmt
	cargo clippy
	cargo test

run: test
	cargo run
clean:
	cargo clean
