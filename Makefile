.PHONY: test
test:
	cargo fmt
	cargo check
	cargo test
	cargo test --release

run: test
	cargo run
