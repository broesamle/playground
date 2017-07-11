SHELL := /bin/bash

all:
	mkdir -p wasm
	cargo build --release --target wasm32-unknown-emscripten
	find target/wasm32-unknown-emscripten/release/deps -type f -name "*.wasm" | xargs -I {} cp {} wasm/alloc_data.wasm
	find target/wasm32-unknown-emscripten/release/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} cp {} wasm/alloc_data.js
