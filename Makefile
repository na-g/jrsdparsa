.PHONY: build

build:
	cargo build --lib --release --target wasm32-unknown-unknown
	wasm-pack build --target web
