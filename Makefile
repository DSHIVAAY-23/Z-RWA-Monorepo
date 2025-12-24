.PHONY: build test optimize

build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test

optimize:
	soroban-cli contract optimize --wasm target/wasm32-unknown-unknown/release/zk_verifier.wasm
