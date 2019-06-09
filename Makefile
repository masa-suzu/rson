.PHONY: test run clean

test: clean
	cargo fmt
	cargo clippy
	cargo test --verbose --all

run: test
	cargo run
clean:
	cargo clean
