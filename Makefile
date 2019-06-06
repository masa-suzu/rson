.PHONY: test
test:
	cargo fmt
	cargo check
	cargo test
	cargo test --release

run: test
	cargo run

build:
	cargo +nightly build --target wasm32-unknown-unknown --release
	mkdir -p docs/dist
	wasm-bindgen ./target/wasm32-unknown-unknown/release/rson.wasm --out-dir docs/dist --nodejs