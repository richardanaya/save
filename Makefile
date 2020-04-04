build:
	cargo build --target=wasm32-wasi --release
	cp target/wasm32-wasi/release/save.wasm .
