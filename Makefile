.PHONY: test run clean

test: clean
	cargo clippy
	cargo fmt -- --check
	cargo test --verbose --all

run: test
	cargo run
clean:
	cargo clean
