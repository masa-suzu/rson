.PHONY: test run clean

test: clean
	cargo clippy
	cargo fmt -- --check
	cargo build --verbose --all
	cargo test --verbose --all
	cargo test --verbose --all --release

run: test
	cargo run
clean:
	cargo clean
